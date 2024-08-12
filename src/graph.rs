use itertools::Itertools;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::iter::zip;

use crate::graph_parser::deserialize_string_to_graph;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct NamedNode {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) weight: i64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Edge {
    pub(crate) u: usize,
    pub(crate) v: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Graph {
    pub(crate) vertices: Vec<NamedNode>,
    pub(crate) edges: Vec<Edge>,
}

impl Ord for NamedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for NamedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Parses a String and converts it to a graph.
impl TryFrom<String> for Graph {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match deserialize_string_to_graph(&value) {
            Ok(graph) => Ok(graph),
            Err(err_tup) => {
                debug!(
                    "Unable to parse string '{}' into graph because of errors.\n1.{}\n2.{}",
                    value, err_tup.0, err_tup.1
                );
                Err("Unable to parse string into graph.")
            }
        }
    }
}

/// Functions to create Graphs from some vertices and there weight.
impl FromIterator<i64> for Graph {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let v = iter.into_iter().collect_vec();
        Graph::from(v)
    }
}

impl From<Vec<i64>> for Graph {
    fn from(value: Vec<i64>) -> Self {
        Graph::new((0..value.len()).map(|x| x.to_string()).collect(), value)
    }
}

impl From<Vec<(String, i64)>> for Graph {
    fn from(value: Vec<(String, i64)>) -> Self {
        Graph::new(
            value.iter().map(|x| x.0.clone()).collect(),
            value.iter().map(|x| x.1).collect(),
        )
    }
}

impl From<HashMap<String, i64>> for Graph {
    fn from(value: HashMap<String, i64>) -> Self {
        Graph::new(
            value.keys().map(|k| k.to_owned()).collect_vec(),
            value.values().map(|w| w.to_owned()).collect_vec(),
        )
    }
}

impl From<Vec<NamedNode>> for Graph {
    fn from(value: Vec<NamedNode>) -> Self {
        let edges = value
            .iter()
            .permutations(2)
            .map(|uv| {
                let u = uv.first().unwrap();
                let v = uv.get(1).unwrap();
                Edge { u: u.id, v: v.id }
            })
            .collect();
        Graph {
            vertices: value,
            edges,
        }
    }
}

impl From<Vec<&NamedNode>> for Graph {
    fn from(value: Vec<&NamedNode>) -> Self {
        let edges = value
            .iter()
            .permutations(2)
            .map(|uv| {
                let u = uv.first().unwrap();
                let v = uv.get(1).unwrap();
                Edge { u: u.id, v: v.id }
            })
            .collect();
        Graph {
            vertices: value.into_iter().map(|x| x.to_owned()).collect(),
            edges,
        }
    }
}

/// Functions to create Graphs from weighted edges.
impl From<HashMap<(String, String), i64>> for Graph {
    fn from(value: HashMap<(String, String), i64>) -> Self {
        let mut unique_v: HashSet<String> = HashSet::new();
        value.keys().for_each(|(s1, s2)| {
            unique_v.insert(s1.to_string());
            unique_v.insert(s2.to_string());
        });
        let mut name_weight_tup: HashMap<String, i64> =
            unique_v.clone().into_iter().map(|x| (x, 0_i64)).collect();
        for uv in unique_v.into_iter().permutations(2) {
            let u: &String = uv.first().unwrap();
            let v: &String = uv.get(1).unwrap();
            let weight = value.get(&(u.to_string(), v.to_string())).unwrap_or(&0);
            if let Some(x) = name_weight_tup.get_mut(u) {
                *x -= weight;
            }
            if let Some(x) = name_weight_tup.get_mut(v) {
                *x += weight;
            }
        }
        Graph::from(name_weight_tup)
    }
}

impl From<Vec<((String, String), i64)>> for Graph {
    fn from(value: Vec<((String, String), i64)>) -> Self {
        let map: HashMap<(String, String), i64> = value.into_iter().collect();
        Graph::from(map)
    }
}

#[allow(clippy::manual_try_fold)]
impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.vertices.iter().fold(Ok(()), |acc, v| {
            acc.and_then(|_| write!(f, "{}: {}; ", &v.name, &v.weight))
        });
        out = out.and_then(|_| writeln!(f));
        out = out.and_then(|_| write!(f, "Edges: "));
        self.edges.iter().fold(out, |acc, e| {
            acc.and_then(|_| write!(f, "{} -> {}; ", &e.u, &e.v))
        })
    }
}

impl Graph {
    pub(crate) fn new(names: Vec<String>, weights: Vec<i64>) -> Self {
        assert!(
            names.len() == weights.len(),
            "The length of the names and weights must be the same."
        );
        let mut vertices: Vec<NamedNode> = vec![];
        let mut edges: Vec<Edge> = vec![];
        let mut id = 0;
        for (name, weight) in zip(names, weights) {
            vertices.push(NamedNode { id, name, weight });
            id += 1;
        }
        for uv in (0..id).permutations(2) {
            let u: usize = *uv.first().unwrap();
            let v: usize = *uv.get(1).unwrap();
            edges.push(Edge { u, v });
        }
        let g = Graph { vertices, edges };
        debug!("Created following graph:\n{}", g.to_string());
        g
    }

    #[allow(dead_code)]
    pub(crate) fn get_node_from_name(&self, s: String) -> Option<&NamedNode> {
        self.vertices.iter().find(|v| v.name == s)
    }

    pub(crate) fn get_node_from_id(&self, id: usize) -> Option<&NamedNode> {
        self.vertices.iter().find(|v| v.id == id)
    }

    pub(crate) fn get_node_name(&self, id: usize) -> Option<String> {
        self.vertices
            .iter()
            .find(|v| v.id == id)
            .map(|v| v.name.clone())
    }

    pub(crate) fn get_node_name_or(&self, id: usize, or: String) -> String {
        self.get_node_name(id).unwrap_or(or)
    }

    pub(crate) fn get_average_vertex_weight(&self) -> f64 {
        self.vertices.iter().map(|v| v.weight).sum::<i64>() as f64 / (self.vertices.len() as f64)
    }
}
