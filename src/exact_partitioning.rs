use itertools::Itertools;
use log::debug;
use std::collections::HashMap;

use crate::approximation::{ApproxomationScheme, GreedySatisfaction, StarExpand};
use crate::graph::{Edge, Graph, NamedNode};
use crate::probleminstance::{ProblemInstance, Solution};
use crate::solver::{SolverApproximation, SolverPartitioning};

impl SolverPartitioning<StarExpand> for ProblemInstance {
    fn solve_via_partitioning(&self) -> Solution {
        solve_with::<StarExpand>(&self.g)
    }
}

impl SolverPartitioning<GreedySatisfaction> for ProblemInstance {
    fn solve_via_partitioning(&self) -> Solution {
        solve_with::<GreedySatisfaction>(&self.g)
    }
}

fn solve_with<S: ApproxomationScheme>(graph: &Graph) -> Solution
where
    ProblemInstance: SolverApproximation<S>,
{
    let mut partitionings = collect_all_partitionigns(&graph.vertices);
    partitionings.sort_by_key(|a| std::cmp::Reverse(a.len()));
    let solution = partitionings.iter().find_map(|x| partition_solver::<S>(x));
    solution
}

fn partition_solver<S: ApproxomationScheme>(
    partitioning: &Vec<Vec<&NamedNode>>,
) -> Solution
where
    ProblemInstance: SolverApproximation<S>,
{
    let mut acc: HashMap<Edge, f64> = HashMap::new();
    for partition in partitioning {
        let g: ProblemInstance = Graph::from(partition.to_vec()).into();
        let result: Solution = g.solve_approx();
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

    use crate::approximation::GreedySatisfaction;
    use crate::exact_partitioning::collect_all_partitionigns;
    use crate::graph::Graph;
    use crate::probleminstance::ProblemInstance;
    use crate::solver::{SolverPartitioning, Solver};
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
        let sol = <dyn SolverPartitioning::<GreedySatisfaction> as Solver>::solve(&instance);
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert!(sol.unwrap().len() == 4);
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
