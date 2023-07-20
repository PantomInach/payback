use graph::Graph;
use ilp::ProblemInstance;

pub mod graph;
pub mod ilp;

fn main() {
    let graph = Graph::from(vec![("A".to_string(), 1), ("B".to_string(), 2), ("C".to_string(), 3)]);
    ProblemInstance::from(graph).run();
}
