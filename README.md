<!--toc:start-->
- [Payback](#payback)
- [How it works](#how-it-works)
  - [Generating Graphs](#generating-graphs)
    - [Via Weighted Directed Edges](#via-weighted-directed-edges)
      - [From Vec<((String, String), i64)>](#from-vecstring-string-i64)
      - [From HashMap<(String, String), i64>](#from-hashmapstring-string-i64)
    - [Via Vertex Weights](#via-vertex-weights)
      - [From Vec\<i64\>](#from-veci64)
      - [From Vec<(String, i64)>](#from-vecstring-i64)
      - [From HashMap<String, i64>](#from-hashmapstring-i64)
  - [Solving](#solving)
- [Note](#note)
<!--toc:end-->

# Payback

If you have a network of people, which own each other money, paying off debts can lead to many transactions. With this crate the amount of transactions can be minimized.

# How it works
We represent the network as a graph. Each node represents one person and every person has an amount of money the need to pay/receive represented as a vertex weight. Negative weights indicate a net dept to the network while positive weights indicate a dept on the side of the network to the person.
The aim is to find directed weighted edges, which indicate cash flow, such that for every person there inflow minus there outflow is equal to their vertex weight (how much money they own/get from the network). Also, the amount of edges should be minimal.

## Generating Graphs
A graph can be generated in two different manners.

### Via Weighted Directed Edges
In this method we describe the edges between the nodes. Each edge has a start and end node with a weight. Every vertex needs a unique name. Otherwise, the two vertices will be interpreted as the same.
We show how to create the following graph.
```mermaid
graph TD;
A -- 1 --> C;
A -- 1 --> D;
B -- 1 --> D;
```
The graph from this representation will just be converted to a graph from [Via Vertex Weights](#via-vertex-weights).

#### From Vec<((String, String), i64)>
```rust
let input: Vec<((String, String), i64)> = vec![
    (("A".to_string(), "C".to_string()), 1),
    (("A".to_string(), "D".to_string()), 1),
    (("B".to_string(), "D".to_string()), 1),
];
let graph: Graph = input.into();
```
Here the nodes are named `A`, `B`, `C`, `D`.

#### From HashMap<(String, String), i64>
```rust
let mut input: HashMap<(String, String), i64> = HashMap::new();
input.insert(("A".to_string(), "C".to_string()), 1);
input.insert(("A".to_string(), "D".to_string()), 1);
input.insert(("B".to_string(), "D".to_string()), 1);
let graph: Graph = input.into();
```
Here the nodes are named `A`, `B`, `C`, `D`.

### Via Vertex Weights
For this method, we will tell the graph the nodes and there weight directly. Here are some options for this option.
We show how to create the following nodes and their weights.
| Vertex | A | B | C | D |
| --- | --- | --- | ---| ---|
| Weight | -2 | -1 | 1 | 2 |

#### From Vec\<i64\>
```rust
let input: Vec<i64> = vec![-2, -1, 1, 2];
let graph: Graph = input.into();
```
Here the nodes names are just numbers from `0` to `n`. Therefore, `0`, `1`, `2`, `3`.

#### From Vec<(String, i64)>
```rust
let input: Vec<(String, i64)> = vec![
    ("A".to_string(), -2),
    ("B".to_string(), -1),
    ("C".to_string(), 1),
    ("D".to_string(), 2),
];
let graph: Graph = input.into();
```
Here the nodes are named `A`, `B`, `C`, `D`.

#### From HashMap<String, i64>
```rust
let mut input: HashMap<String, i64> = HashMap::new();
input.insert("A".to_string(), -2);
input.insert("B".to_string(), -1);
input.insert("C".to_string(), 1);
input.insert("D".to_string(), 2);
let graph: Graph = input.into();
```
Here the nodes are named `A`, `B`, `C`, `D`.

## Solving
Available solver:
| Solver | Type | Description |
| --- | --- | --- |
| Star Expand | 2 Approximation | Approximates optimal solution by choosing central node, to which all edges are incident. |
| Greedy Satisfaction | 2 Approximation | Approximates optimal solution while minimizing the total weight of all edges. |
| Partitioning with Star Expand | Exact | Partitioning based exact solver, which solves base cases with Star Expand. |
| Partitioning with Greedy Satisfaction | Exact | Partitioning based exact solver, which solves base cases with Greedy Satisfaction. |

Approximation algorithm don't necessarily return the optimal solution but theirs is not worse than a given factor. Also, they run in polynomial time.

Exact algorithm give the optimal solution, but its runtime is not polynomial. This can lead to long runtimes while working with larger inputs. Generally it is uncommon to have an instance, for which an approximation algorithm does not return the optimal answer.

### Using Star Expand
```rust
let instance = ProblemInstance::from(graph);
let sol = <dyn SolverApproximation::<StarExpand> as Solver>::solve(&instance);
instance.print_solution(sol);
```

### Using Greedy Satisfaction 
```rust
let instance = ProblemInstance::from(graph);
let sol = <dyn SolverApproximation::<GreedySatisfaction> as Solver>::solve(&instance);
instance.print_solution(sol);
```

### Using Partitioning with Star Expand
```rust
let instance = ProblemInstance::from(graph);
let sol = <dyn SolverPartitioning::<StarExpand> as Solver>::solve(&instance);
instance.print_solution(sol);
```

### Using Partitioning with Greedy Satisfaction
```rust
let instance = ProblemInstance::from(graph);
let sol = <dyn SolverPartitioning::<GreedySatisfaction> as Solver>::solve(&instance);
instance.print_solution(sol);
```

# Note
This problem is NP-Hard and therefore can have a long runtime for bigger instances.
