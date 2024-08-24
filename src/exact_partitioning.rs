use itertools::Itertools;
use log::debug;
use std::collections::HashMap;

use crate::graph::{Edge, Graph, NamedNode};
use crate::probleminstance::{ProblemInstance, Solution};

/// Algorithm solving the payback problem naivly by iteration all possible partitionings of the
/// vertices. Has a runtime of O^*(n^n / (ln n)^n). Should not be used.
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
/// let solution: Solution = instance.solve_with(SolvingMethods::PartitioningStarExpand);
/// ```
pub(crate) fn naive_all_partitioning(
    instance: &ProblemInstance,
    approx_solver: &dyn Fn(&ProblemInstance) -> Solution,
) -> Solution {
    let mut partitionings = collect_all_partitionigns(&instance.g.vertices);
    partitionings.sort_by_key(|a| std::cmp::Reverse(a.len()));
    let solution = partitionings
        .iter()
        .find_map(|x| partition_solver(x, approx_solver));
    solution
}

fn partition_solver(
    partitioning: &Vec<Vec<&NamedNode>>,
    approx_solver: &dyn Fn(&ProblemInstance) -> Solution,
) -> Solution {
    let mut acc: HashMap<Edge, f64> = HashMap::new();
    for partition in partitioning {
        let instance: ProblemInstance = Graph::from(partition.to_vec()).into();
        let result: Solution = approx_solver(&instance);
        match result {
            Some(map) => {
                acc.extend(map);
            }
            None => {
                debug!(
                    "Partitioning {:?} failed due to partition {:?}",
                    partitioning, partition
                );
                return None;
            }
        }
    }
    debug!(
        "Found solution for partitioning {:?}. Edges: {:?}",
        partitioning, acc
    );
    Some(acc)
}

fn collect_all_partitionigns<'a, T>(items: &'a [T]) -> Vec<Vec<Vec<&'a T>>> {
    let mut acc: Vec<Vec<Vec<&'a T>>> = Vec::new();
    iterate_all_partitionings(&mut Vec::new(), items, &mut |x| {
        acc.push(x.to_owned());
    });
    acc
}

fn iterate_all_partitionings<'a, T, F>(head: &mut Vec<Vec<&'a T>>, rest: &'a [T], f: &mut F)
where
    F: FnMut(&mut Vec<Vec<&'a T>>),
{
    if rest.is_empty() {
        f(head)
    } else {
        let (first, tail) = rest.split_at(1);
        for i in 0..head.len() {
            if let Some(x) = head.get_mut(i) {
                x.append(&mut first.iter().collect_vec());
            }
            iterate_all_partitionings(head, tail, f);
            if let Some(x) = head.get_mut(i) {
                x.pop();
            }
        }
        head.push(first.iter().collect_vec());
        iterate_all_partitionings(head, tail, f);
        head.pop();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::approximation::{greedy_satisfaction, star_expand};
    use crate::exact_partitioning::collect_all_partitionigns;
    use crate::exact_partitioning::naive_all_partitioning;
    use crate::graph::Graph;
    use crate::probleminstance::ProblemInstance;
    use env_logger::Env;
    use log::debug;

    fn init() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_perfect_matching_sol() {
        init();
        debug!("Running 'test_perfect_matching_sol'");
        let graph: Graph = vec![-1, -1, 1, 1, 2, -2, 3, -3].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = naive_all_partitioning(&instance, &greedy_satisfaction);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert!(sol.unwrap().len() == 4);

        let graph: Graph = vec![6, 3, 2, 1, -4, -8].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = naive_all_partitioning(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 4);

        let graph: Graph = vec![6, 3, 2, 1, -4, -8, 0].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = naive_all_partitioning(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert_eq!(sol.unwrap().len(), 4);
    }

    #[test]
    fn test_partitionings() {
        init();
        debug!("Running 'test_partitionings'");
        let v: Vec<i64> = vec![1, 2, 3];
        let acc = collect_all_partitionigns(&v);
        debug!("All partitionings of '{:?}': {:?}", v, acc);
        assert!(acc.len() == 5);
        let calulated: HashSet<Vec<Vec<&i64>>> = acc.into_iter().collect();
        let res: HashSet<Vec<Vec<&i64>>> = vec![
            vec![vec![&1, &2, &3]],
            vec![vec![&1], vec![&2, &3]],
            vec![vec![&1, &2], vec![&3]],
            vec![vec![&1, &3], vec![&2]],
            vec![vec![&1], vec![&2], vec![&3]],
        ]
        .into_iter()
        .collect();
        assert_eq!(calulated, res);

        let v: Vec<i64> = vec![1, 2, 3, 4];
        let acc = collect_all_partitionigns(&v);
        debug!("All partitionings of '{:?}': {:?}", v, acc);
        let res: HashSet<Vec<Vec<&i64>>> = vec![
            vec![vec![&1, &2, &3, &4]],
            vec![vec![&1], vec![&2, &3, &4]],
            vec![vec![&1, &2], vec![&3, &4]],
            vec![vec![&1, &3, &4], vec![&2]],
            vec![vec![&1], vec![&2], vec![&3, &4]],
            vec![vec![&1, &2, &3], vec![&4]],
            vec![vec![&1, &4], vec![&2, &3]],
            vec![vec![&1], vec![&2, &3], vec![&4]],
            vec![vec![&1, &3], vec![&2, &4]],
            vec![vec![&1, &2, &4], vec![&3]],
            vec![vec![&1], vec![&2, &4], vec![&3]],
            vec![vec![&1, &2], vec![&3], vec![&4]],
            vec![vec![&1, &3], vec![&2], vec![&4]],
            vec![vec![&1, &4], vec![&2], vec![&3]],
            vec![vec![&1], vec![&2], vec![&3], vec![&4]],
        ]
        .into_iter()
        .collect();
        let calulated: HashSet<Vec<Vec<&i64>>> = acc.into_iter().collect();
        assert_eq!(calulated, res);
    }
}
