use log::debug;
use petgraph::{dot::Dot, graph::DiGraph, graph::NodeIndex};
use std::collections::HashMap;

use crate::approximation::{greedy_satisfaction, star_expand};
use crate::dynamic_program::patcas_dp;
use crate::exact_partitioning::naive_all_partitioning;
use crate::graph::{Edge, Graph, NamedNode};
use crate::tree_bases::best_partition;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub type Solution = Option<HashMap<Edge, f64>>;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum SolvingMethods {
    /// 2-Approximation schema with one high responsibility node.
    /// Doesn't necessarily return minimal total transaction amount possible.
    ApproxStarExpand,
    /// 2-Approximation schema with minimal edge weight sum.
    ApproxGreedySatisfaction,
    /// Excat partitioning based solving algorithmus, which solves partitions with 'StarExpand'.
    /// Doesn't necessarily return minimal total transaction amount possible.
    PartitioningStarExpand,
    /// Excat partitioning based solving algorithmus, which solves partitions with
    /// 'GreedySatisfaction'.
    PartitioningGreedySatisfaction,
    /// Branching based algorithm running in O*(3^n) time, which solves partitions with 'StarExpand'.
    /// Doesn't necessarily return minimal total transaction amount possible.
    BranchingPartitionStarExpand,
    /// Branching based algorithm running in O*(3^n) time, which solves partitions with 'GreedySatisfaction'.
    BranchingPartitionGreedySatisfaction,
    /// Dynamic program with a runtime of O*(3^n), which solves partitions with 'StarExpand'.
    /// Doesn't necessarily return minimal total transaction amount possible.
    DPStarExpand,
    /// Dynamic program with a runtime of O*(3^n), which solves partitions with 'GreedySatisfaction'.
    DPGreedySatisfaction,
}

pub struct ProblemInstance {
    pub(crate) g: Graph,
}

impl From<Graph> for ProblemInstance {
    fn from(value: Graph) -> Self {
        ProblemInstance::new(value)
    }
}

#[allow(dead_code)]
impl ProblemInstance {
    fn new(graph: Graph) -> Self {
        ProblemInstance { g: graph }
    }

    pub fn is_solvable(&self) -> bool {
        let avg = self.g.get_average_vertex_weight();
        if avg != 0_f64 {
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

    pub fn solve_with(&self, method: SolvingMethods) -> Solution {
        match method {
            SolvingMethods::ApproxStarExpand => star_expand(self),
            SolvingMethods::ApproxGreedySatisfaction => greedy_satisfaction(self),
            SolvingMethods::PartitioningStarExpand => naive_all_partitioning(self, &star_expand),
            SolvingMethods::PartitioningGreedySatisfaction => {
                naive_all_partitioning(self, &greedy_satisfaction)
            }
            SolvingMethods::BranchingPartitionStarExpand => best_partition(self, &star_expand),
            SolvingMethods::BranchingPartitionGreedySatisfaction => {
                best_partition(self, &greedy_satisfaction)
            }
            SolvingMethods::DPStarExpand => patcas_dp(self, &star_expand),
            SolvingMethods::DPGreedySatisfaction => patcas_dp(self, &greedy_satisfaction),
        }
    }

    pub(crate) fn optimal_transaction_amount(&self) -> i64 {
        self.g.vertices.iter().map(|v| v.weight.abs()).sum::<i64>() / 2
    }

    pub fn solution_string(&self, solution: &Solution) -> Result<String, String> {
        match solution {
            None => Err("No result was found.".to_string()),
            Some(map) => {
                let mut res: String = "".to_string();
                for (edge, weight) in map {
                    let u = self.g.get_node_name_or(edge.u, edge.u.to_string());
                    let v = self.g.get_node_name_or(edge.v, edge.v.to_string());
                    if *weight >= 0.0 {
                        res += &format!("{:?} to {:?}: {:?}", v, u, weight);
                    } else {
                        res += &format!("{:?} to {:?}: {:?}", u, v, -weight);
                    }
                    res += LINE_ENDING;
                }
                Ok(res)
            }
        }
    }

    pub fn solution_to_dot_string(&self, solution: &Solution) -> Result<String, String> {
        match solution {
            None => {
                println!("No result was found.");
                Err("No result was found.".to_owned())
            }
            Some(sol) => {
                let mut pet_graph =
                    DiGraph::<String, f64>::with_capacity(self.g.vertices.len(), sol.len());
                let node_map: HashMap<NamedNode, NodeIndex> = self
                    .g
                    .vertices
                    .iter()
                    .map(|v| (v.to_owned(), pet_graph.add_node(v.name.to_owned())))
                    .collect();
                sol.iter().try_for_each(|(e, w)| -> Result<(), String> {
                    let u = self
                        .g
                        .get_node_from_id(e.u)
                        .ok_or(format!("Can't find vertex with index {:?}", e.u))
                        .and_then(|u_node| {
                            node_map.get(u_node).ok_or(format!(
                                "Can't find node '{:?}' in the pet graph.",
                                u_node.name
                            ))
                        })?;
                    let v = self
                        .g
                        .get_node_from_id(e.v)
                        .ok_or(format!("Can't find vertex with index {:?}", e.v))
                        .and_then(|v_node| {
                            node_map.get(v_node).ok_or(format!(
                                "Can't find node '{:?}' in the pet graph.",
                                v_node.name
                            ))
                        })?;
                    pet_graph.update_edge(v.to_owned(), u.to_owned(), *w);
                    Ok(())
                })?;
                Ok(Dot::new(&pet_graph).to_string())
            }
        }
    }
}
