use clap::{Args, Parser};

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}

#[derive(Parser,Debug)]
struct Arguments {
    // mandatory args (must have higher indexes)
    #[arg(short = 'm')]
    mandatory_arg: usize,

    //exclusive arg group
    #[command(flatten)]
    exclusive_arg_group: ExclusiveArgGroup,

    // optional args
    #[arg(short = 'o')]
    optional_arg: Option<usize>,

}

#[derive(Args,Debug)]
#[group(required = true, multiple = false)]
struct ExclusiveArgGroup{
    #[arg(short = 'a')]
    arg_a : bool,

    #[arg(short = 'b')]
    arg_b : bool,

}


// arg relations tutorial 
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#argument-relations