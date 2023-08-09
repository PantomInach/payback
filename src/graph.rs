use itertools::Itertools;
use log::debug;
use std::{
    collections::{HashMap, HashSet},
    iter::zip, borrow::BorrowMut,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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
        match (self.weight, other.weight) {
            (u, v) if u > v => Some(std::cmp::Ordering::Greater),
            (u, v) if u == v => Some(std::cmp::Ordering::Equal),
            (u, v) if u < v => Some(std::cmp::Ordering::Less),
            (_, _) => None,
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

/// Functions to create Graphs from weighted edges.
impl From<HashMap<(String, String), i64>> for Graph {
    fn from(value: HashMap<(String, String), i64>) -> Self {
        let mut unique_v: HashSet<String> = HashSet::new();
        value.keys().for_each(|(s1, s2)| {
            unique_v.insert(s1.to_string());
            unique_v.insert(s2.to_string());
        });
        let mut name_weight_tup: HashMap<String, i64> = unique_v
            .clone()
            .into_iter()
            .map(|x| (x, 0 as i64))
            .collect();
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

    pub(crate) fn edge_weight_upper_bound(&self) -> i64 {
        self.vertices.iter().map(|node| node.weight).sum()
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

    pub(crate) fn edges_into(&self, v: usize) -> Vec<Edge> {
        self.edges
            .iter()
            .filter(|e| e.v == v)
            .cloned()
            .collect_vec()
    }

    pub(crate) fn edges_out(&self, u: usize) -> Vec<Edge> {
        self.edges
            .iter()
            .filter(|e| e.u == u)
            .cloned()
            .collect_vec()
    }

    // pub(crate) fn get_node_name_from_node(&self, v: NamedNode) -> Option<String> {
    //     self.get_node_name(v.id)
    // }

    pub(crate) fn to_string(&self) -> String {
        let mut out: String = "Vertices:".to_string();
        out = self.vertices.iter().fold(out, |acc, v| {
            acc + " " + &v.name.to_string() + ": " + &v.weight.to_string() + ";"
        });
        out += "\nEdges:";
        out = self.edges.iter().fold(out, |acc, e| {
            acc + " " + &e.u.to_string() + " -> " + &e.v.to_string() + ";"
        });
        out
    }

    pub(crate) fn induce(&self, vertices: Vec<usize>) -> Graph {
        let new_vertices: Vec<NamedNode> = self
            .vertices
            .clone()
            .into_iter()
            .filter(|v| vertices.contains(&v.id))
            .collect();
        Graph::from(new_vertices)
    }
}
