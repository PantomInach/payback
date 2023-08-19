use crate::graph::Edge;
use crate::approximation::ApproxomationScheme;
use std::collections::HashMap;

pub(crate) trait Solver {
    fn solve(&self) -> Option<HashMap<Edge, f64>>;
}

pub(crate) trait SolverApproximation<S: ApproxomationScheme> {
    fn solve_approx(&self) -> Option<HashMap<Edge, f64>>;
}

pub(crate) trait SolverExact {
    fn solve_exact(&self) -> Option<HashMap<Edge, f64>>;
}

pub(crate) trait SolverPartitioning<S>
where
    S: ApproxomationScheme,
{
    fn solve_via_partitioning(&self) -> Option<HashMap<Edge, f64>>;
}

impl<S: ApproxomationScheme> Solver for dyn SolverApproximation<S> {
    fn solve(&self) -> Option<HashMap<Edge, f64>> {
        self.solve_approx()
    }
}

impl Solver for dyn SolverExact {
    fn solve(&self) -> Option<HashMap<Edge, f64>> {
        self.solve_exact()
    }
}

impl<S: ApproxomationScheme> SolverExact for dyn SolverPartitioning<S>{
    fn solve_exact(&self) -> Option<HashMap<Edge, f64>> {
        self.solve_via_partitioning()
    }
}

impl<S: ApproxomationScheme> Solver for dyn SolverPartitioning<S> {
    fn solve(&self) -> Option<HashMap<Edge, f64>> {
        self.solve_via_partitioning()
    }
}
