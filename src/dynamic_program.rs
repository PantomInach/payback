use std::collections::HashMap;

use itertools::Itertools;
use log::debug;

use crate::{
    graph::{Edge, Graph, NamedNode},
    probleminstance::{ProblemInstance, Solution},
};

type Table = HashMap<(u128, u128), (usize, Option<(u128, u128)>)>;

/// Algorithm solving the payback problem via a dynamic program.
/// Based on algorithm by [Patcas](https://www.cs.ubbcluj.ro/~studia-i/contents/2009-2/10-Patcas.pdf).
/// The algorithm has a runtime of O^*(3^n).
///
/// * `instance` - The problem instance which should be solved
/// * `approx_solver` - Approximation algorithm used to solve partition, which have no zero sum
/// subset
///
/// Example:
/// ```
/// use payback::graph::Graph;
/// use payback::probleminstance::{ProblemInstance, Solution, SolvingMethods};
///
/// let instance: ProblemInstance = Graph::from(vec![-2, -1, 1, 2]).into();
/// let solution: Solution = instance.solve_with(SolvingMethods::DPStarExpand);
/// ```
pub(crate) fn patcas_dp(
    instance: &ProblemInstance,
    approx_solver: &dyn Fn(&ProblemInstance) -> Solution,
) -> Solution {
    if !instance.is_solvable() {
        return None;
    }

    // Initialise all needed data for pre and post processing.
    let index_to_node: HashMap<usize, &NamedNode> = instance
        .g
        .vertices
        .iter()
        .filter(|v| v.weight != 0)
        .enumerate()
        .collect();
    let weights: Vec<i64> = index_to_node
        .iter()
        .sorted_by(|(i1, _), (i2, _)| i1.cmp(i2))
        .map(|(_, v)| v.weight)
        .collect_vec();
    // Initialise the algorithms parameters.
    let (v_left, v_right): (Vec<_>, Vec<_>) =
        index_to_node.iter().partition(|(_, n)| n.weight >= 0);
    debug!("Left and right nodes: {:?} ----- {:?}", v_left, v_right);
    let n_left: u128 = expand_number(&v_left.into_iter().map(|(i, _)| *i).collect_vec());
    let n_right: u128 = expand_number(&v_right.into_iter().map(|(i, _)| *i).collect_vec());
    let table: &mut Table = &mut HashMap::new();
    // Execute the dynamic program.
    let _ = dp(n_left, n_right, &weights, table);

    let solution_partition = table_extract_partitioning(n_left, n_right, table)
        .into_iter()
        .map(|x| {
            one_indices(x)
                .into_iter()
                .map(|i| index_to_node[&i])
                .collect_vec()
        })
        .collect_vec();
    debug!(
        "Patcas_dp proposes following partitioning: {:?}",
        solution_partition
    );

    let solution: &mut HashMap<Edge, f64> = &mut HashMap::new();
    solution_partition
        .into_iter()
        .map(|s| approx_solver(&ProblemInstance::from(Graph::from(s))))
        .for_each(|sol| {
            match sol {
                Some(m) => solution.extend(m),
                None => unreachable!("The instance is solvable and the recursion should have only added zero sum subsets."),
            }
        });
    Some(solution.to_owned())
}

/// Underlying dynamic program for [`patcas_dp()`].
fn dp(i: u128, j: u128, weights: &Vec<i64>, table: &mut Table) -> Option<usize> {
    debug!("Calling dp with {:?}, {:?}", i, j);
    if i == 0 && j == 0 {
        return Some(0);
    }

    if let Some((x, _)) = table.get(&(i, j)) {
        debug!("Table hit -> {:?}", x);
        return Some(*x);
    }

    if number_weight(i, weights) != -number_weight(j, weights) {
        debug!(
            "Number weight is not the same: {} VS {}",
            number_weight(i, weights),
            number_weight(j, weights)
        );
        return None;
    }
    debug!(
        "{} and {} have the same weight of {}",
        i,
        j,
        number_weight(i, weights)
    );

    let value = number_and_subset(i)
        .cartesian_product(number_and_subset(j).collect_vec())
        .flat_map(|(a, b)| {
            let val = dp(i ^ a, j ^ b, weights, table).map(|x| {
                (
                    x + a.count_ones() as usize + b.count_ones() as usize - 1,
                    (i != a && j != b).then_some((a, b)),
                )
            });
            debug!(
                "Size for i: {}, j: {}, a: {}, b: {} -> {:?}",
                i, j, a, b, val
            );
            val
        })
        .min_by(|(x, _), (y, _)| x.cmp(y));
    debug!("Minimum partitioning given with: {:?}", value);
    if let Some(v) = value {
        table.insert((i, j), v);
    }
    value.map(|v| v.0)
}

/// For a given table from [`dp()`] this function backtracks the table to finde the corresponding
/// partitioning from the starting point of (i, j).
fn table_extract_partitioning(i: u128, j: u128, table: &Table) -> Vec<u128> {
    debug!(
        "Beginning partitioning extraction with i: {}, j: {} for table: {:?}",
        i, j, table
    );
    let partitions: &mut Vec<u128> = &mut vec![];
    _table_extract_rec(i, j, table, partitions);
    partitions.to_owned()
}

fn _table_extract_rec(i: u128, j: u128, table: &Table, partitions: &mut Vec<u128>) {
    if i == 0 || j == 0 {
        return;
    }
    match table.get(&(i, j)) {
        Some((_, None)) => partitions.push(i + j),
        Some((_, Some((a, b)))) => {
            _table_extract_rec(*a, *b, table, partitions);
            _table_extract_rec(i ^ a, j ^ b, table, partitions)
        }
        _ => (),
    }
}

/// For every position the number has a one in its binary representation, get the corresponding
/// weight from the same position and add them all up.
fn number_weight(num: u128, weights: &[i64]) -> i64 {
    // TODO: Test if faster with chaching.
    assert!((u128::BITS - num.leading_zeros()) as usize <= weights.len());
    let mut i = 0;
    let mut n = num;
    let mut sol: i64 = 0;
    while n > 0 {
        if n % 2 == 1 {
            sol += weights[i];
        }
        i += 1;
        n >>= 1;
    }
    sol
}

/// Returns a vec of numbers n where num AND n == n and n != 0.
fn number_and_subset(num: u128) -> impl Iterator<Item = u128> {
    one_indices(num)
        .into_iter()
        .powerset()
        .map(|s| expand_number(&s))
        .filter(move |n| *n != 0)
}

/// Returns the indices of a one digit in the binary representation of the given number.
fn one_indices(num: u128) -> Vec<usize> {
    let mut i = 0;
    let mut n = num;
    let mut indices: Vec<usize> = vec![];
    while n > 0 {
        if n % 2 == 1 {
            indices.push(i);
        }
        i += 1;
        n >>= 1;
    }
    indices
}

/// Constructs a number with a one in the binary representation at the given indices.
fn expand_number(indices: &[usize]) -> u128 {
    assert!(indices.len() <= 128);
    let mut num: u128 = 0;
    for index in indices {
        num += 1 << index;
    }
    num
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{dp, expand_number, number_and_subset, one_indices, Table};
    use crate::approximation::star_expand;
    use crate::dynamic_program::{number_weight, patcas_dp};
    use crate::graph::Graph;
    use crate::probleminstance::ProblemInstance;
    use env_logger::Env;
    use itertools::Itertools;
    use log::debug;

    fn init() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_expand_number_one_indices() {
        let num = 0b100110;
        assert_eq!(one_indices(num), vec![1, 2, 5]);
        assert_eq!(num, expand_number(&one_indices(num)));
    }

    #[test]
    fn test_number_weight() {
        let weights: Vec<i64> = vec![1, 2, 4, 8, 16, 32, 64, 128];
        for i in 0..255 {
            assert_eq!(number_weight(i as u128, &weights), i);
        }
    }

    #[test]
    fn test_number_and_subset() {
        let num = 0b110;
        let mut expected_res: Vec<u128> = vec![0b100, 0b010, 0b110];
        expected_res.sort();
        let mut result = number_and_subset(num).collect_vec();
        result.sort();
        assert_eq!(result, expected_res);
    }

    #[test]
    fn test_dp() {
        let i = 0b1100;
        let j = 0b0011;
        let weights = vec![2, 1, -1, -2];
        let table: &mut Table = &mut HashMap::new();
        dp(i, j, &weights, table);
        assert!(table.get(&(i, j)).is_some());
        assert_eq!(table.get(&(i, j)).unwrap().0, 2);
    }

    #[test]
    fn test_patcas_dp() {
        init();
        let graph: Graph = vec![-1, -1, 1, 1, 2, -2, 3, -3].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = patcas_dp(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 4);

        let graph: Graph = vec![-2, -1, 1, 1, 2, -2, 3, -3].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = patcas_dp(&instance, &star_expand);
        assert!(sol.is_none());

        let graph: Graph = vec![6, 3, 2, 1, -4, -8].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = patcas_dp(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 4);

        let graph: Graph = vec![6, 3, 2, 1, -4, -8, 0].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = patcas_dp(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 4);

        let graph: Graph = vec![1, 1, 1, 1, 1, 1, -6].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = patcas_dp(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 6);

        let graph: Graph = vec![9, 4, 1, -6, -6, -2].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = patcas_dp(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 5);
    }
}
