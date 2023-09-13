use csv::ReaderBuilder;
use itertools::Itertools;
use serde_derive::Deserialize;

use crate::graph::Graph;

#[derive(Debug, PartialEq, Deserialize)]
struct NodeRecord {
    name: String,
    weight: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
struct EdgeRecord {
    from: String,
    to: String,
    weight: i64,
}

impl NodeRecord {
    fn to_tuple(&self) -> (String, i64) {
        (self.name.to_owned(), self.weight)
    }
}

impl EdgeRecord {
    fn to_tuple(&self) -> ((String, String), i64) {
        ((self.from.to_owned(), self.to.to_owned()), self.weight)
    }
}

pub(crate) fn deserialize_string_to_graph(
    data: &String,
) -> Result<Graph, (csv::Error, csv::Error)> {
    let node_deserialized = deserialize_to_nodes(data)
        .map(|nodes| Into::<Graph>::into(nodes.iter().map(|n| n.to_tuple()).collect_vec()));
    if let Ok(graph) = node_deserialized {
        return Ok(graph);
    }
    let edge_deserialized = deserialize_to_edges(data)
        .map(|edges| Into::<Graph>::into(edges.iter().map(|n| n.to_tuple()).collect_vec()));
    if let Ok(graph) = edge_deserialized {
        Ok(graph)
    } else {
        Err((
            node_deserialized.unwrap_err(),
            edge_deserialized.unwrap_err(),
        ))
    }
}

fn deserialize_to_nodes(data: &String) -> Result<Vec<NodeRecord>, csv::Error> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    rdr.deserialize().collect()
}

fn deserialize_to_edges(data: &String) -> Result<Vec<EdgeRecord>, csv::Error> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    rdr.deserialize().collect()
}

#[cfg(test)]
mod tests {
    use env_logger::Env;
    use log::debug;

    use crate::graph_parser::{deserialize_to_edges, deserialize_to_nodes, EdgeRecord, NodeRecord};

    fn init() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_deserialize_to_nodes() {
        init();
        debug!("Running 'test_deserialize_to_nodes'");
        let data = "A,-1\nB,2\nC,-1";
        let out = deserialize_to_nodes(&data.to_string());
        assert!(out.is_ok());
        assert_eq!(
            out.unwrap(),
            vec![
                NodeRecord {
                    name: "A".to_string(),
                    weight: -1
                },
                NodeRecord {
                    name: "B".to_string(),
                    weight: 2
                },
                NodeRecord {
                    name: "C".to_string(),
                    weight: -1
                }
            ]
        );
        let data = "A,C,1";
        assert!(deserialize_to_nodes(&data.to_string()).is_err());
    }

    #[test]
    fn test_deserialize_to_edges() {
        init();
        debug!("Running 'test_deserialize_to_edges'");
        let data = "A,B,1\nB,C,1\nC,A,1";
        let out = deserialize_to_edges(&data.to_string());
        assert!(out.is_ok());
        assert_eq!(
            out.unwrap(),
            vec![
                EdgeRecord {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    weight: 1
                },
                EdgeRecord {
                    from: "B".to_string(),
                    to: "C".to_string(),
                    weight: 1
                },
                EdgeRecord {
                    from: "C".to_string(),
                    to: "A".to_string(),
                    weight: 1
                }
            ]
        );
        let data = "A,1";
        assert!(deserialize_to_edges(&data.to_string()).is_err());
    }
}
