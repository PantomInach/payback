use crate::graph::{Edge, Graph};
use log::debug;
use std::collections::HashMap;

pub(crate) struct ProblemInstance {
    pub(crate) g: Graph,
}

impl From<Graph> for ProblemInstance {
    fn from(value: Graph) -> Self {
        ProblemInstance::new(value)
    }
}

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

    pub(crate) fn optimal_transaction_amount(&self) -> i64 {
        self.g.vertices.iter().map(|v| v.weight.abs()).sum()
    }

    pub(crate) fn print_solution(&self, solution: Option<HashMap<Edge, f64>>) {
        match solution {
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
