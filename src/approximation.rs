use log::debug;
use std::collections::HashMap;

use crate::graph::{Edge, NamedNode};
use crate::probleminstance::{ProblemInstance, Solution};

/// Algorithm approximating the payback problem by building a tree.
/// Has a approximation factor of 2. The proposed solution has at most twice as many edges as the
/// optimum.
/// Does NOT necessarily return the solution with a minimal total transaction amount.
/// The algorithm has a linear runtime.
///
/// * `instance` - The problem instance which should be solved
/// * `approx_solver` - Approximation algorithm used to solve partition, which have no zero sum
/// subset
///
/// Example:
/// ```
/// use payback::graph::Graph;
/// use payback::probleminstance::{ProblemInstance, Solution, SolvingMethods};
///
/// let instance: ProblemInstance = Graph::from(vec![-2, -1, 1, 2]).into();
/// let solution: Solution = instance.solve_with(SolvingMethods::ApproxStarExpand);
/// ```
pub(crate) fn star_expand(instance: &ProblemInstance) -> Solution {
    debug!(
        "Running 'star_expand' for graph: {:?}",
        instance.g.to_string()
    );
    if !instance.is_solvable() {
        None
    } else {
        let mut total_transaction_amount = 0;
        let v_max: Option<&NamedNode> = instance.g.vertices.iter().max();
        match v_max {
            None => None,
            Some(v) => {
                let edges: HashMap<Edge, f64> = instance
                    .g
                    .vertices
                    .iter()
                    .filter(|u| u != &v)
                    .map(|u| {
                        total_transaction_amount += u.weight.abs();
                        if u.weight > 0 {
                            (Edge { u: u.id, v: v.id }, u.weight as f64)
                        } else {
                            (Edge { u: v.id, v: u.id }, -u.weight as f64)
                        }
                    })
                    .collect();
                debug!(
                    "Caculated Approximation for graph {:?} with edges {:?}",
                    instance.g.to_string(),
                    edges
                );
                debug!(
                    "The total amount of transactions is: {:?} VS Optimum: {:?}",
                    total_transaction_amount,
                    instance.optimal_transaction_amount()
                );
                Some(edges)
            }
        }
    }
}

/// Algorithm approximating the payback problem by greedily building edges in a bipartite graph.
/// Has a approximation factor of 2. The proposed solution has at most twice as many edges as the
/// optimum.
/// Returns a solution with minimal total transaction amount.
/// The algorithm has a linear runtime.
///
/// * `instance` - The problem instance which should be solved
/// * `approx_solver` - Approximation algorithm used to solve partition, which have no zero sum
/// subset
///
/// Example:
/// ```
/// use payback::graph::Graph;
/// use payback::probleminstance::{ProblemInstance, Solution, SolvingMethods};
///
/// let instance: ProblemInstance = Graph::from(vec![-2, -1, 1, 2]).into();
/// let solution: Solution = instance.solve_with(SolvingMethods::ApproxGreedySatisfaction);
/// ```
pub(crate) fn greedy_satisfaction(instance: &ProblemInstance) -> Solution {
    debug!(
        "Running 'greedy_satisfaction' for graph: {:?}",
        instance.g.to_string()
    );
    if !instance.is_solvable() {
        None
    } else {
        let mut sol = HashMap::new();
        let (mut neg_vertices, mut pos_vertices): (Vec<&NamedNode>, Vec<&NamedNode>) =
            instance.g.vertices.iter().partition(|v| v.weight < 0_i64);
        let mut side_capacities = 0;
        if let Some(x) = neg_vertices.first() {
            side_capacities = x.weight;
        }
        if let Some(x) = pos_vertices.first() {
            if x.weight > side_capacities.abs() {
                side_capacities = x.weight;
            }
        }
        while !neg_vertices.is_empty() && !pos_vertices.is_empty() {
            let n = neg_vertices.first().unwrap();
            let p = pos_vertices.first().unwrap();
            match side_capacities.cmp(&0_i64) {
                std::cmp::Ordering::Less => {
                    if p.weight <= -side_capacities {
                        sol.insert(Edge { u: p.id, v: n.id }, p.weight as f64);
                        side_capacities += p.weight;
                        if side_capacities == 0 {
                            neg_vertices.remove(0);
                        }
                        pos_vertices.remove(0);
                    } else {
                        sol.insert(Edge { u: p.id, v: n.id }, side_capacities as f64);
                        side_capacities += p.weight;
                        neg_vertices.remove(0);
                    }
                }
                std::cmp::Ordering::Equal => {
                    side_capacities = p.weight;
                }
                std::cmp::Ordering::Greater => {
                    if -n.weight <= side_capacities {
                        sol.insert(Edge { u: p.id, v: n.id }, n.weight.abs() as f64);
                        side_capacities += n.weight;
                        if side_capacities == 0 {
                            pos_vertices.remove(0);
                        }
                        neg_vertices.remove(0);
                    } else {
                        sol.insert(Edge { u: p.id, v: n.id }, side_capacities as f64);
                        side_capacities += n.weight;
                        pos_vertices.remove(0);
                    }
                }
            }
        }
        Some(sol)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::approximation::greedy_satisfaction;
    use crate::approximation::star_expand;
    use crate::graph::Edge;
    use crate::graph::Graph;
    use crate::probleminstance::ProblemInstance;
    use env_logger::Env;
    use log::debug;

    fn init() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_star_expand() {
        init();
        debug!("Running 'test_star_expand'");
        let graph: Graph = vec![
            ("A".to_owned(), -2_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstance = graph.into();
        let sol = star_expand(&instance);
        debug!(
            "For graph '{:?}' star_expand returns: {:?}",
            graph_string, sol
        );
        assert!(sol.is_none());

        let graph: Graph = vec![
            ("A".to_owned(), -1_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstance = graph.clone().into();
        let sol_opt = star_expand(&instance);
        debug!(
            "For graph '{:?}' star_expand returns: {:?}",
            graph_string, sol
        );
        assert!(sol_opt.is_some());
        let na = instance.g.get_node_from_name("A".to_owned()).unwrap();
        let nb = instance.g.get_node_from_name("B".to_owned()).unwrap();
        let nc = instance.g.get_node_from_name("C".to_owned()).unwrap();
        let nd = instance.g.get_node_from_name("D".to_owned()).unwrap();
        let res: HashMap<Edge, f64> = HashMap::from([
            (Edge { u: nc.id, v: na.id }, 1.0_f64),
            (Edge { u: nc.id, v: nd.id }, 4.0_f64),
            (Edge { u: nb.id, v: nc.id }, 2.0_f64),
        ]);
        let sol = sol_opt.unwrap();
        debug!("Solution:        {:?}", sol);
        debug!("Expected Result: {:?}", res);
        for (e, w) in sol {
            assert!(
                res.contains_key(&e),
                "Edge '{:?}' should not be in the solution.",
                e
            );
            let rew = res.get(&e).unwrap().to_owned();
            assert!(
                rew == w,
                "Edge '{:?}' should have a weight of '{:?}' but has '{:?}'",
                e,
                rew,
                w
            );
        }
    }

    #[test]
    fn test_greedy_satisfaction() {
        init();
        debug!("Running 'test_greedy_satisfaction'");
        let graph: Graph = vec![
            ("A".to_owned(), -2_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstance = graph.into();
        let sol = greedy_satisfaction(&instance);
        debug!(
            "For graph '{:?}' greedy_satisfaction returns: {:?}",
            graph_string, sol
        );
        assert!(sol.is_none());

        let graph: Graph = vec![
            ("A".to_owned(), -1_i64),
            ("B".to_owned(), 2_i64),
            ("C".to_owned(), 3_i64),
            ("D".to_owned(), -4_i64),
        ]
        .into();
        let graph_string = graph.to_string();
        let instance: ProblemInstance = graph.into();
        let sol = greedy_satisfaction(&instance);
        debug!(
            "For graph '{:?}' greedy_satisfaction returns: {:?}",
            graph_string, sol
        );
        assert!(sol.is_some());
        assert_eq!(sol.unwrap().into_iter().map(|(_, v)| v).sum::<f64>(), 5_f64);
    }
}
