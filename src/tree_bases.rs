use std::collections::HashMap;

use crate::graph::{Edge, Graph, NamedNode};
use crate::probleminstance::{ProblemInstance, Solution};
use itertools::Itertools;

/// Algorithm solving the payback problem via a branching based approach.
///
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
/// let solution: Solution = instance.solve_with(SolvingMethods::BranchingPartitionStarExpand);
/// ```
pub(crate) fn best_partition(
    instance: &ProblemInstance,
    approx_solver: &dyn Fn(&ProblemInstance) -> Solution,
) -> Solution {
    if !instance.is_solvable() {
        return None;
    }
    let current_sol: &mut Vec<Vec<NamedNode>> = &mut vec![];
    let solution_partition: Vec<Vec<NamedNode>> =
        best_partition_rec(&instance.g.vertices, current_sol);
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

fn best_partition_rec(
    vertices: &[NamedNode],
    current_sol: &mut Vec<Vec<NamedNode>>,
) -> Vec<Vec<NamedNode>> {
    if vertices.is_empty() {
        return current_sol.to_vec();
    }
    let remove_verts: &mut Vec<&NamedNode> = &mut vec![];
    let subsets = zero_sum_subsets(vertices);
    let filtered_subsets = subsets
        .iter()
        .filter(|s| match s.len() {
            1 => {
                remove_verts.push(s.first().unwrap());
                false
            }
            2 => {
                let u = s.first().unwrap();
                let v = s.last().unwrap();
                if !remove_verts.contains(&u) && !remove_verts.contains(&v) {
                    current_sol.push(vec![u.clone(), v.clone()]);
                    remove_verts.push(u);
                    remove_verts.push(v);
                }
                false
            }
            _ => true,
        })
        .collect_vec();
    filtered_subsets.into_iter().fold(vec![], |acc, s| {
        let verts = vertices
            .iter()
            .filter(|v| !s.contains(v) && !remove_verts.contains(v))
            .cloned()
            .collect_vec();
        let result = best_partition_rec(&verts, current_sol);
        if result.len() >= acc.len() {
            result
        } else {
            acc
        }
    });
    current_sol.to_vec()
}

fn zero_sum_subsets(vertices: &[NamedNode]) -> Vec<Vec<NamedNode>> {
    vertices
        .iter()
        .powerset()
        .filter(|s| s.iter().map(|n| n.weight).sum::<i64>() == 0)
        .map(|s| s.into_iter().cloned().collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::approximation::star_expand;
    use crate::graph::Graph;
    use crate::probleminstance::ProblemInstance;
    use crate::tree_bases::best_partition;
    use env_logger::Env;
    use log::debug;

    fn init() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_best_partition() {
        init();
        debug!("Running 'test_best_partition'");
        let graph: Graph = vec![-1, -1, 1, 1, 2, -2, 3, -3].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = best_partition(&instance, &star_expand);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert!(sol.unwrap().len() == 4);

        let graph: Graph = vec![-2, -1, 1, 1, 2, -2, 3, -3].into();
        debug!("Using graph: {:?}", graph);
        let instance = ProblemInstance::from(graph);
        let sol = best_partition(&instance, &star_expand);
        assert!(sol.is_none());
    }
}