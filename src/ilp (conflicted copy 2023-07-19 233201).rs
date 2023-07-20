use std::collections::HashMap;
use std::iter::Map;

use good_lp::{Constraint, Expression, ProblemVariables, VariableDefinition};
use itertools::Itertools;

use crate::graph::Graph;

pub struct ProblemInstance {
    g: Graph,
    x_vars_def: HashMap<(usize, usize), VariableDefinition>,
    w_vars_def: HashMap<(usize, usize), VariableDefinition>,
}

pub fn build_vars(g: Graph) -> ProblemVariables {
    let x_vars_def = build_x_vars(&g);
    let w_vars_def = build_w_vars(&g);
    let mut vars: ProblemVariables = ProblemVariables::new();
    x_vars_def.values().for_each(|var| {
        vars.add(var.clone());
    });
    w_vars_def.values().for_each(|var| {
        vars.add(var.clone());
    });
    vars
}

pub fn build_constraints(prob: ProblemInstance) -> Vec<Constraint> {
    let mut contraints: Vec<Constraint> = vec![];
    for edge in prob.g.edges {}
    contraints
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
    });
    vars
}
