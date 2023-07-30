use std::collections::HashMap;
use log::debug;

use good_lp::{
    default_solver, Constraint, Expression, ProblemVariables, ResolutionError, Solution,
    SolverModel, Variable, VariableDefinition,
};

use crate::graph::Graph;

pub(crate) struct ProblemInstance {
    g: Graph,
    x_vars: HashMap<(usize, usize), Variable>,
    w_vars: HashMap<(usize, usize), Variable>,
    vars: ProblemVariables,
}

impl From<Graph> for ProblemInstance {
    fn from(g: Graph) -> Self {
        let mut x_vars: HashMap<(usize, usize), Variable> = HashMap::new();
        let mut w_vars: HashMap<(usize, usize), Variable> = HashMap::new();
        let mut vars: ProblemVariables = ProblemVariables::new();
        for ((u, v), var_def) in build_x_vars(&g) {
            x_vars.insert((u, v), vars.add(var_def));
        }
        for ((u, v), var_def) in build_w_vars(&g) {
            w_vars.insert((u, v), vars.add(var_def));
        }
        ProblemInstance {
            g,
            x_vars,
            w_vars,
            vars,
        }
    }
}

impl From<Vec<u64>> for ProblemInstance {
    fn from(value: Vec<u64>) -> Self {
        ProblemInstance::from(Graph::from(value))
    }
}

impl From<Vec<(String, u64)>> for ProblemInstance {
    fn from(value: Vec<(String, u64)>) -> Self {
        ProblemInstance::from(Graph::from(value))
    }
}

fn interpret_solution(w_vars: HashMap<(usize, usize), Variable>, graph: Graph, sol: impl Solution) {
    for ((u, v), w_x) in w_vars {
        let res = sol.value(w_x);
        // if res == 0.0 {
        //     continue;
        // }
        println!(
            "{:?} to {:?}: {:?}",
            graph.get_node_name(u).unwrap_or(u.to_string()),
            graph.get_node_name(v).unwrap_or(v.to_string()),
            res
        );
    }
}

impl ProblemInstance {
    pub fn run(self) {
        let w_vars = self.w_vars.clone();
        let graph = self.g.clone();
        let sol = self.solve();
        match sol {
            Ok(s) => interpret_solution(w_vars, graph, s),
            Err(_) => println!("No result was found."),
        }
    }

    fn solve(self) -> Result<impl Solution, ResolutionError> {
        let min_expression = self.build_min_expression();
        let x_constraints = self.build_constraints_x();
        let w_constraints = self.build_constraints_vertex();
        let mut instance = self.vars.minimise(min_expression).using(default_solver);
        for con in x_constraints {
            instance = instance.with(con);
        }
        for con in w_constraints {
            instance = instance.with(con);
        }
        instance.solve()
    }

    /// Generates the minimizing target sum_(e in E) x_e
    fn build_min_expression(&self) -> Expression {
        let mut min_expr: Expression = 0.into();
        self.x_vars
            .values()
            .into_iter()
            .for_each(|var| min_expr += var);
        min_expr
    }

    /// Generates the contraints for defining x_e depending on w_e
    /// So
    ///     x_e <= w_e <==> w_e - x_e >= 0
    ///     w_e <= U * x_e <==> U * x_e - w_e >= 0
    fn build_constraints_x(&self) -> Vec<Constraint> {
        let mut constraints: Vec<Constraint> = vec![];
        let upper_bound: f64 = self.g.edge_weight_upper_bound() as f64;
        for edge in &self.g.edges {
            let mut expr_first: Expression = 0.into();
            let mut expr_second: Expression = 0.into();
            let w_e: &Variable = self.w_vars.get(&(edge.u, edge.v)).unwrap();
            let x_e: &Variable = self.x_vars.get(&(edge.u, edge.v)).unwrap();
            // w_e - x_e >= 0
            expr_first.add_mul(1.0, w_e);
            expr_first.add_mul(-1.0, x_e);
            constraints.push(expr_first.geq(0.0));
            // U * x_e - w_e >= 0
            expr_second.add_mul(upper_bound, x_e);
            expr_second.add_mul(-1.0, w_e);
            constraints.push(expr_second.geq(0.0));
        }
        constraints
    }

    /// Generates the contraint w(v) + In - Out = avg_weight_of_nodes
    ///     for all v in V: w(v) + sum_((u,v) in E) w_(u,v) - sum_((v,u) in E) w_e
    fn build_constraints_vertex(&self) -> Vec<Constraint> {
        let mut constraints: Vec<Constraint> = vec![];
        let avg: f64 = self.g.get_average_vertex_weight();
        for v in self.g.vertices.iter().clone() {
            let vars_in: Vec<&Variable> = self
                .g
                .edges_into(v.id)
                .iter()
                .map(|e| self.w_vars.get(&(e.u, e.v)).unwrap())
                .collect();
            let vars_out: Vec<&Variable> = self
                .g
                .edges_out(v.id)
                .iter()
                .map(|e| self.w_vars.get(&(e.u, e.v)).unwrap())
                .collect();
            println!("{:?}", vars_out);
            println!("{:?}", vars_in);
            let mut constraint: Expression = (v.weight as f64).into();
            vars_in
                .into_iter()
                .for_each(|w_in| constraint.add_mul(1.0, w_in));
            vars_out
                .into_iter()
                .for_each(|w_out| constraint.add_mul(-1.0, w_out));
            constraints.push(constraint.eq(avg));
        }
        constraints
    }
}

fn build_x_vars(g: &Graph) -> HashMap<(usize, usize), VariableDefinition> {
    let mut vars: HashMap<(usize, usize), VariableDefinition> = HashMap::new();
    g.edges.iter().for_each(|edge| {
        let name_u = g.get_node_name_or(edge.u, edge.u.to_string());
        let name_v = g.get_node_name_or(edge.v, edge.v.to_string());
        vars.insert(
            (edge.u, edge.v),
            VariableDefinition::new()
                .binary()
                .name(format!("x_({}, {})", name_u, name_v)),
        );
        debug!("Created binary Var x_({}, {})", name_u, name_v);
    });
    vars
}

fn build_w_vars(g: &Graph) -> HashMap<(usize, usize), VariableDefinition> {
    let mut vars: HashMap<(usize, usize), VariableDefinition> = HashMap::new();
    let upper_bound: f64 = g.edge_weight_upper_bound() as f64;
    g.edges.iter().for_each(|edge| {
        let name_u = g.get_node_name_or(edge.u, edge.u.to_string());
        let name_v = g.get_node_name_or(edge.v, edge.v.to_string());
        vars.insert(
            (edge.u, edge.v),
            VariableDefinition::new()
                .integer()
                .min(0)
                .max(upper_bound)
                .name(format!("w_({}, {})", name_u, name_v)),
        );
        debug!("Created integer Var w_({}, {}) from 0 to {}", name_u, name_v, upper_bound);
    });
    vars
}
