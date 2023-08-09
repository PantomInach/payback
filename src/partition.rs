use std::collections::HashMap;

use crate::graph::{Edge, Graph, NamedNode};
use log::{debug, info};

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

    fn star_expand(&self) -> Option<HashMap<Edge, f64>> {
        // debug!("Running 'star_expand' for graph: {:?}", self.g.to_string());
        let avg = self.g.get_average_vertex_weight();
        debug!("Avaerage should be 0. Average of current graph: {:?}", avg);
        if avg != 0.0 {
            debug!(
                "Graph {:?} has not the average weight {:?}. Should be 0",
                self.g.to_string(),
                avg
            );
            None
        } else {
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
                            if u > v {
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
                    Some(edges)
                }
            }
        }
    }

    fn partition_solver(&self, partitioning: &[&[NamedNode]]) -> Option<HashMap<Edge, f64>> {
        let mut acc: HashMap<Edge, f64> = HashMap::new();
        for partition in partitioning {
            let g: ProblemInstancePartition = ProblemInstancePartition {
                g: Graph::from(partition.to_vec()),
            };
            match g.star_expand() {
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

    pub fn solve(&self) {
        info!("Running partition based solver...");
        let solution: Option<HashMap<Edge, f64>> =
            iterate_all_subdivisions(&mut Vec::new(), &self.g.vertices, &mut |x| {
                self.partition_solver(x)
            });
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

fn iterate_all_subdivisions<'a, T, F>(
    head: &mut Vec<&'a [T]>,
    rest: &'a [T],
    f: &mut F,
) -> Option<HashMap<Edge, f64>>
where
    F: FnMut(&[&[T]]) -> Option<HashMap<Edge, f64>>,
{
    if rest.is_empty() {
        f(head)
    } else {
        for i in 1..=rest.len() {
            let (next, tail) = rest.split_at(i);
            head.push(next);
            if let Some(x) = iterate_all_subdivisions(head, tail, f) {
                return Some(x);
            }
            head.pop();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::partition::iterate_all_subdivisions;
    use crate::Graph;
    use crate::ProblemInstancePartition;
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
        let graph: Graph = vec![-1, -1, 1, 1].into();
        debug!("{:?}", graph);
        let instance = ProblemInstancePartition::from(graph);
        let sol = iterate_all_subdivisions(&mut Vec::new(), &instance.g.vertices, &mut |x| {
            instance.partition_solver(x)
        });
        println!("{:?}", sol);
        assert!(sol.is_some());
        assert!(sol.unwrap().len() == 2);
    }
}
