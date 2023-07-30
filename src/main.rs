use graph::Graph;
use ilp::ProblemInstance;

pub mod graph;
pub mod ilp;

fn main() {
    env_logger::init();
    let graph = Graph::from(vec![("A".to_string(), 1), ("B".to_string(), 2), ("C".to_string(), 3)]);
    // let graph = Graph::from(vec![("A".to_string(), 1), ("B".to_string(), 3)]);
    ProblemInstance::from(graph).run();
}
