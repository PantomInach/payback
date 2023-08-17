use std::collections::HashMap;

use crate::graph::{Edge, Graph, NamedNode};
use itertools::Itertools;
use log::{debug, info};

#[derive(Debug, Clone, Copy)]
pub enum PartitionSolvingMethod {
    StarExpand,
    GreedySatsifaction,
}

pub(crate) struct ProblemInstancePartition {
    g: Graph,
}

impl From<Graph> for ProblemInstancePartition {
    fn from(value: Graph) -> Self {
        ProblemInstancePartition::new(value)
    }
}

impl ProblemInstancePartition {
    fn new(g: Graph) -> Self {
        Self { g }
    }

    fn is_solvable(&self) -> bool {
        let avg = self.g.get_average_vertex_weight();
        if self.g.get_average_vertex_weight() != 0_f64 {
            debug!(
                "Graph {:?} has not the average weight {:?}. Should be 0",
                self.g.to_string(),
                avg
            );
            false
        } else {
            true
        }
    }

    /// Can lead to unoptimal esge weights
    fn star_expand(&self) -> Option<HashMap<Edge, f64>> {
        debug!("Running 'star_expand' for graph: {:?}", self.g.to_string());
        if !self.is_solvable() {
            None
        } else {
            let mut total_transaction_amount = 0;
            let v_max: Option<&NamedNode> = self.g.vertices.iter().max();
            match v_max {
                None => None,
                Some(v) => {
                    let edges: HashMap<Edge, f64> = self
                        .g
                        .vertices
                        .iter()
                        .filter(|u| u != &v)
                        .map(|u| {
                            total_transaction_amount += u.weight.abs();
                            if u.weight > 0 {
                                (Edge { u: u.id, v: v.id }, u.weight as f64)
                            } else {
                                (Edge { u: v.id, v: u.id }, -u.weight as f64)
                            }
                        })
                        .collect();
                    debug!(
                        "Caculated Approximation for graph {:?} with edges {:?}",
                        self.g.to_string(),
                        edges
                    );
                    debug!(
                        "The total amount of transactions is: {:?}\nVS Optimum: {:?}",
                        total_transaction_amount,
                        self.optimal_transaction_amount()
                    );
                    Some(edges)
                }
            }
        }
    }

    fn greedy_satisfaction(&self) -> Option<HashMap<Edge, f64>> {
        debug!(
            "Running 'greedy_satisfaction' for graph: {:?}",
            self.g.to_string()
        );
        if !self.is_solvable() {
            None
        } else {
            let mut sol = HashMap::new();
            let (mut neg_vertices, mut pos_vertices): (Vec<&NamedNode>, Vec<&NamedNode>) =
                self.g.vertices.iter().partition(|v| v.weight < 0_i64);
            let mut side_capacities = 0;
            if let Some(x) = neg_vertices.first() {
                side_capacities = x.weight;
            }
            if let Some(x) = pos_vertices.first() {
                if x.weight > side_capacities.abs() {
                    side_capacities = x.weight;
                }
            }
            while !neg_vertices.is_empty() && !pos_vertices.is_empty() {
                let n = neg_vertices.first().unwrap();
                let p = pos_vertices.first().unwrap();
                match side_capacities.cmp(&0_i64) {
                    std::cmp::Ordering::Less => {
                        if p.weight <= -side_capacities {
                            sol.insert(Edge { u: p.id, v: n.id }, p.weight as f64);
                            side_capacities += p.weight;
                            if side_capacities == 0 {
                                neg_vertices.remove(0);
                            }
                            pos_vertices.remove(0);
                        } else {
                            sol.insert(Edge { u: p.id, v: n.id }, side_capacities as f64);
                            side_capacities += p.weight;
                            neg_vertices.remove(0);
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        side_capacities = p.weight;
                    }
                    std::cmp::Ordering::Greater => {
                        if -n.weight <= side_capacities {
                            sol.insert(Edge { u: p.id, v: n.id }, n.weight.abs() as f64);
                            side_capacities += n.weight;
                            if side_capacities == 0 {
                                pos_vertices.remove(0);
                            }
                            neg_vertices.remove(0);
                        } else {
                            sol.insert(Edge { u: p.id, v: n.id }, side_capacities as f64);
                            side_capacities += n.weight;
                            pos_vertices.remove(0);
                        }
                    }
                }
            }
            Some(sol)
        }
    }

    fn optimal_transaction_amount(&self) -> i64 {
        self.g.vertices.iter().map(|v| v.weight.abs()).sum()
    }

    fn partition_solver(
        &self,
        partitioning: &Vec<Vec<&NamedNode>>,
        solving_method: PartitionSolvingMethod,
    ) -> Option<HashMap<Edge, f64>> {
        let mut acc: HashMap<Edge, f64> = HashMap::new();
        for partition in partitioning {
            let g: ProblemInstancePartition = ProblemInstancePartition {
                g: Graph::from(partition.to_vec()),
            };
            let result = match solving_method {
                PartitionSolvingMethod::StarExpand => g.star_expand(),
                PartitionSolvingMethod::GreedySatsifaction => g.greedy_satisfaction(),
                _ => panic!(
                    "Solving method '{:?}' was not implemented in the partition_solver method.",
                    solving_method
                ),
            };
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

    pub fn solve(&self) -> Option<HashMap<Edge, f64>> {
        self.solve_with(PartitionSolvingMethod::GreedySatsifaction)
    }

    pub fn solve_with(&self, solving_method: PartitionSolvingMethod) -> Option<HashMap<Edge, f64>> {
        info!(
            "Running partition based solver with solving_method '{:?}'...",
            solving_method
        );
        let mut partitionings = collect_all_partitionigns(&self.g.vertices);
        partitionings.sort_by_key(|a| std::cmp::Reverse(a.len()));
        let solution: Option<HashMap<Edge, f64>> = partitionings
            .iter()
            .find_map(|x| self.partition_solver(x, solving_method));
        solution
    }

    pub fn solve_and_interpret(&self) {
        self.solve_and_interpret_with(PartitionSolvingMethod::GreedySatsifaction)
    }
    
    pub fn solve_and_interpret_with(&self, solving_method: PartitionSolvingMethod) {
        match self.solve_with(solving_method) {
            None => println!("No result was found."),
            Some(map) => {
                for (edge, weight) in map {
                    let u = self.g.get_node_name_or(edge.u, edge.u.to_string());
                    let v = self.g.get_node_name_or(edge.v, edge.v.to_string());
                    if weight >= 0.0 {
                        println!("{:?} to {:?}: {:?}", u, v, weight);
                    } else {
                        println!("{:?} to {:?}: {:?}", v, u, -weight);
                    }
                }
            }
        }
    }
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
    use std::collections::HashMap;
    use std::collections::HashSet;

    use crate::graph::Edge;
    use crate::partition::collect_all_partitionigns;
    use crate::graph::Graph;
    use crate::partition::ProblemInstancePartition;
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
        let instance = ProblemInstancePartition::from(graph);
        let sol = instance.solve();
        assert!(sol.is_some());
        debug!("Proposed solution by solver: {:?}", sol);
        assert!(sol.unwrap().len() == 4);
    }

    #[test]
    fn test_star_expand() {
        init();
        debug!("Running 'test_star_expand'");
        let graph: Graph = vec![
            ("A".to_owned(), -2_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstancePartition = graph.into();
        let sol = instance.star_expand();
        debug!(
            "For graph '{:?}' star_expand returns: {:?}",
            graph_string, sol
        );
        assert!(instance.star_expand().is_none());

        let graph: Graph = vec![
            ("A".to_owned(), -1_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstancePartition = graph.clone().into();
        let sol_opt = instance.star_expand();
        debug!(
            "For graph '{:?}' star_expand returns: {:?}",
            graph_string, sol
        );
        assert!(sol_opt.is_some());
        let na = instance.g.get_node_from_name("A".to_owned()).unwrap();
        let nb = instance.g.get_node_from_name("B".to_owned()).unwrap();
        let nc = instance.g.get_node_from_name("C".to_owned()).unwrap();
        let nd = instance.g.get_node_from_name("D".to_owned()).unwrap();
        let res: HashMap<Edge, f64> = HashMap::from([
            (Edge { u: nc.id, v: na.id }, 1.0_f64),
            (Edge { u: nc.id, v: nd.id }, 4.0_f64),
            (Edge { u: nb.id, v: nc.id }, 2.0_f64),
        ]);
        let sol = sol_opt.unwrap();
        debug!("Solution:        {:?}", sol);
        debug!("Expected Result: {:?}", res);
        for (e, w) in sol {
            assert!(
                res.contains_key(&e),
                "Edge '{:?}' should not be in the solution.",
                e
            );
            let rew = res.get(&e).unwrap().to_owned();
            assert!(
                rew == w,
                "Edge '{:?}' should have a weight of '{:?}' but has '{:?}'",
                e,
                rew,
                w
            );
        }
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

    #[test]
    fn test_greedy_satisfaction() {
        init();
        debug!("Running 'test_greedy_satisfaction'");
        let graph: Graph = vec![
            ("A".to_owned(), -2_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstancePartition = graph.into();
        let sol = instance.greedy_satisfaction();
        debug!(
            "For graph '{:?}' greedy_satisfaction returns: {:?}",
            graph_string, sol
        );
        assert!(instance.star_expand().is_none());

        let graph: Graph = vec![
            ("A".to_owned(), -1_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstancePartition = graph.into();
        let sol = instance.greedy_satisfaction();
        debug!(
            "For graph '{:?}' greedy_satisfaction returns: {:?}",
            graph_string, sol
        );
        assert!(sol.is_some());
        assert_eq!(sol.unwrap().into_iter().map(|(_, v)| v).sum::<f64>(), 5_f64);
    }
}
