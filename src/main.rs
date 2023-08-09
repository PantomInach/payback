use std::collections::HashMap;

use graph::Graph;
use ilp::ProblemInstance;
use partition::ProblemInstancePartition;

pub mod graph;
pub mod ilp;
pub mod partition;

fn main() {
    env_logger::init();
    // let graph = Graph::from(vec![("A".to_string(), 1), ("B".to_string(), 2), ("C".to_string(), 3)]);
    // let graph = Graph::from(vec![("Josy".to_string(), 516), ("John".to_string(), 12108), ("Dobi".to_string(), 874), ("Niki".to_string(), 0), ("Philip".to_string(), 2420), ("Steff".to_string(), 3189)]);
    // ProblemInstance::from(graph).run();
    let graph: Graph = vec![
        (("Josy".to_string(), "John".to_string()), 4194),
        (("Philip".to_string(), "John".to_string()), 2498),
        (("Niki".to_string(), "John".to_string()), 4665),
        (("Steff".to_string(), "John".to_string()), 376),
        (("Dobi".to_string(), "John".to_string()), 375),
        (("Niki".to_string(), "Dobi".to_string()), 376),
        (("Niki".to_string(), "Steff".to_string()), 3198),
        (("Niki".to_string(), "Philip".to_string()), 2375),
        (("Josy".to_string(), "Philip".to_string()), 45),
        (("Josy".to_string(), "Niki".to_string()), 471),
    ]
    .into();
    ProblemInstancePartition::from(graph).solve();
}
