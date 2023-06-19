use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add(Arguments),

    /// Removes files from myapp
    Remove(Arguments),
}

#[derive(Parser, Debug, Clone)]
struct Arguments {
    // mandatory args (must have higher indexes)
    #[arg(short = 'm')]
    mandatory_arg: usize,

    // exclusive arg group
    #[command(flatten)]
    exclusive_arg_group: ExclusiveArgGroup,

    // optional args
    #[arg(short = 'o')]
    optional_arg: Option<usize>,
}

#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
struct ExclusiveArgGroup {
    // a XOR b
    #[arg(short = 'a')]
    arg_a: bool,

    #[arg(short = 'b')]
    arg_b: bool,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add(args) => {
            println!("{:?}", args);
        }

        Commands::Remove(args) => {
            println!("{:?}", args);
        }
    }
}

// arg relations tutorial
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#argument-relations
