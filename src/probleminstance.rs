use log::debug;
use petgraph::{dot::Dot, graph::DiGraph, graph::NodeIndex};
use std::collections::HashMap;

use crate::approximation::{GreedySatisfaction, StarExpand};
use crate::graph::{Edge, Graph, NamedNode};
use crate::solver::{Solver, SolverApproximation, SolverPartitioning};

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub(crate) type Solution = Option<HashMap<Edge, f64>>;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub(crate) enum SolvingMethods {
    /// 2-Approximation schema with one high responsibility node.
    /// Doesn't necessarily return minimal edge weight sum.
    ApproxStarExpand,
    /// 2-Approximation schema with minimal edge weight sum.
    ApproxGreedySatisfaction,
    /// Excat partitioning based solving algorithmus, which solves partitions with 'StarExpand'.
    PartitioningStarExpand,
    /// Excat partitioning based solving algorithmus, which solves partitions with
    /// 'GreedySatisfaction'.
    PartitioningGreedySatisfaction,
}

pub(crate) struct ProblemInstance {
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

    pub(crate) fn is_solvable(&self) -> bool {
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

    pub(crate) fn solve_with(&self, method: SolvingMethods) -> Solution {
        match method {
            SolvingMethods::ApproxStarExpand => {
                <dyn SolverApproximation<StarExpand> as Solver>::solve(self)
            }
            SolvingMethods::ApproxGreedySatisfaction => {
                <dyn SolverApproximation<GreedySatisfaction> as Solver>::solve(self)
            }
            SolvingMethods::PartitioningStarExpand => {
                <dyn SolverPartitioning<StarExpand> as Solver>::solve(self)
            }
            SolvingMethods::PartitioningGreedySatisfaction => {
                <dyn SolverPartitioning<GreedySatisfaction> as Solver>::solve(self)
            }
        }
    }

    pub(crate) fn optimal_transaction_amount(&self) -> i64 {
        self.g.vertices.iter().map(|v| v.weight.abs()).sum()
    }

    pub(crate) fn solution_string(&self, solution: &Solution) -> Result<String, String> {
        match solution {
            None => Err("No result was found.".to_string()),
            Some(map) => {
                let mut res: String = "".to_string();
                for (edge, weight) in map {
                    let u = self.g.get_node_name_or(edge.u, edge.u.to_string());
                    let v = self.g.get_node_name_or(edge.v, edge.v.to_string());
                    if *weight >= 0.0 {
                        res += &format!("{:?} to {:?}: {:?}", u, v, weight);
                    } else {
                        res += &format!("{:?} to {:?}: {:?}", v, u, -weight);
                    }
                    res += LINE_ENDING;
                }
                Ok(res)
            }
        }
    }

    pub(crate) fn solution_to_dot_string(&self, solution: &Solution) -> Result<String, String> {
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
                sol.iter()
                    .try_for_each(|(e, w)| -> Result<(), String> {
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
                        pet_graph.update_edge(u.to_owned(), v.to_owned(), *w);
                        Ok(())
                    })?;
                Ok(Dot::new(&pet_graph).to_string())
            }
        }
    }
}
