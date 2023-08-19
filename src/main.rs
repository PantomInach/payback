use approximation::GreedySatisfaction;
use graph::Graph;
use probleminstance::ProblemInstance;
use solver::{SolverPartitioning, Solver};

use crate::{approximation::StarExpand, solver::SolverApproximation};

pub mod graph;
pub mod probleminstance;
pub mod approximation;
pub mod solver;
pub mod exact_partitioning;

fn main() {
    env_logger::init();
    let graph: Graph = vec![
        (("B".to_string(), "A".to_string()), 4194),
        (("D".to_string(), "A".to_string()), 2498),
        (("C".to_string(), "A".to_string()), 4665),
        (("E".to_string(), "A".to_string()), 376),
        (("F".to_string(), "A".to_string()), 375),
        (("C".to_string(), "F".to_string()), 376),
        (("C".to_string(), "E".to_string()), 3198),
        (("C".to_string(), "D".to_string()), 2375),
        (("B".to_string(), "D".to_string()), 45),
        (("B".to_string(), "C".to_string()), 471),
    ]
    .into();
    let instance = ProblemInstance::from(graph);
    let sol = <dyn SolverPartitioning::<GreedySatisfaction> as Solver>::solve(&instance);
    println!("Exact solution with Greedy Satisfaction:");
    instance.print_solution(sol);
    let sol = <dyn SolverPartitioning::<StarExpand> as Solver>::solve(&instance);
    println!("Exact solution with Star Expand:");
    instance.print_solution(sol);
    let sol = <dyn SolverApproximation::<GreedySatisfaction> as Solver>::solve(&instance);
    println!("Approx solution with Greedy Satisfaction:");
    instance.print_solution(sol);
    let sol = <dyn SolverApproximation::<StarExpand> as Solver>::solve(&instance);
    println!("Approx solution with Star Expand:");
    instance.print_solution(sol);
}
