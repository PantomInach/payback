use crate::approximation::ApproxomationScheme;
use crate::probleminstance::Solution;

pub(crate) trait Solver {
    fn solve(&self) -> Solution;
}

pub(crate) trait SolverApproximation<S: ApproxomationScheme> {
    fn solve_approx(&self) -> Solution;
}

pub(crate) trait SolverExact {
    fn solve_exact(&self) -> Solution;
}

pub(crate) trait SolverPartitioning<S>
where
    S: ApproxomationScheme,
{
    fn solve_via_partitioning(&self) -> Solution;
}

impl<S: ApproxomationScheme> Solver for dyn SolverApproximation<S> {
    fn solve(&self) -> Solution {
        self.solve_approx()
    }
}

impl Solver for dyn SolverExact {
    fn solve(&self) -> Solution {
        self.solve_exact()
    }
}

impl<S: ApproxomationScheme> SolverExact for dyn SolverPartitioning<S>{
    fn solve_exact(&self) -> Solution {
        self.solve_via_partitioning()
    }
}

impl<S: ApproxomationScheme> Solver for dyn SolverPartitioning<S> {
    fn solve(&self) -> Solution {
        self.solve_via_partitioning()
    }
}
