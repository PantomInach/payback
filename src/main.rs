use clap::{Parser, ValueEnum};
use clap_stdin::FileOrStdin;
use env_logger::Env;
use graph::Graph;
use probleminstance::{ProblemInstance, SolvingMethods};

pub mod approximation;
pub mod exact_partitioning;
pub mod graph;
pub mod probleminstance;
pub mod solver;
pub mod graph_parser;

/// Test
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify input via file
    #[arg(group = "input")]
    file: FileOrStdin,

    /// Turns on verbose output
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Turn on debug output
    #[arg(short = 'd', long)]
    debug: bool,

    /// Output method
    #[arg(value_enum, default_value_t = OutputFormat::Transactions)]
    output: OutputFormat,

    /// Specify solving method
    #[arg(value_enum, default_value_t = SolvingMethods::ApproxStarExpand)]
    method: SolvingMethods,
}

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    /// Dot format for graphviz
    Dot,
    /// Print result to stdout by listing the needed transactions
    Transactions,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let log_level = match (args.verbose, args.debug) {
        (_, true) => "debug",
        (true, _) => "info",
        (_, _) => "off",
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();
    let graph: Graph = args.file.to_string().try_into()?;
    let instance = ProblemInstance::from(graph);
    let sol = instance.solve_with(args.method);
    let out = match args.output {
        OutputFormat::Dot => instance.solution_to_dot_string(&sol),
        OutputFormat::Transactions => instance.solution_string(&sol),
    };
    match out {
        Ok(s) => {
            println!("{}", s);
            Ok(())
        },
        Err(s) => {
            println!("Error: {}", s);
            Err(s)
        },
    }
}
