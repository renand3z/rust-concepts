use clap::{Parser, Subcommand};
mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod structs2;
mod structs3;
mod tuples;
mod types;
mod vars;
mod vectors;

#[derive(Parser, Clone)]
#[clap()]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}


#[derive(Subcommand, Clone)]
enum Commands {
  Print,
  Vars,
  Types,
  Strings,
  Tuples,
  Structs2,
  Arrays,
  Vectors,
  Conditionals,
  Loops,
  Functions,
  PointerRef,
  Structs,
  Enums,
  Cli,
  Structs3
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Print => print::run(),
        Commands::Vars => vars::run(),
        Commands::Types => types::run(),
        Commands::Strings => strings::run(),
        Commands::Tuples => tuples::run(),
        Commands::Structs2 => structs2::run(),
        Commands::Arrays => arrays::run(),
        Commands::Vectors => vectors::run(),
        Commands::Conditionals => conditionals::run(),
        Commands::Loops => loops::run(),
        Commands::Functions => functions::run(),
        Commands::PointerRef => pointer_ref::run(),
        Commands::Structs => structs::run(),
        Commands::Enums => enums::run(),
        Commands::Cli => cli::run(),
        Commands::Structs3 => structs3::run(),
    }
}
