use graph::Graph;
use partition::ProblemInstancePartition;

pub mod graph;
pub mod partition;

fn main() {
    env_logger::init();
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
