use itertools::Itertools;
use std::iter::zip;

#[derive(Clone)]
pub(crate) struct NamedNode {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) weight: u64,
}

#[derive(Clone)]
pub(crate) struct Edge {
    pub(crate) u: usize,
    pub(crate) v: usize,
}

#[derive(Clone)]
pub(crate) struct Graph {
    pub(crate) vertices: Vec<NamedNode>,
    pub(crate) edges: Vec<Edge>,
}

impl From<Vec<u64>> for Graph {
    fn from(value: Vec<u64>) -> Self {
        Graph::new((0..value.len()).map(|x| x.to_string()).collect(), value)
    }
}

impl From<Vec<(String, u64)>> for Graph {
    fn from(value: Vec<(String, u64)>) -> Self {
        Graph::new(
            value.iter().map(|x| x.0.clone()).collect(),
            value.iter().map(|x| x.1).collect(),
        )
    }
}

impl Graph {
    pub(crate) fn new(names: Vec<String>, weights: Vec<u64>) -> Self {
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
        Graph { vertices, edges }
    }

    pub(crate) fn edge_weight_upper_bound(&self) -> u64 {
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

    pub(crate) fn get_average_vertex_weight(&self) -> u64 {
        self.vertices.iter().map(|v| v.weight).sum()
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

    pub(crate) fn get_node_name_from_node(&self, v: NamedNode) -> Option<String> {
        self.get_node_name(v.id)
    }
}
