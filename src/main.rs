mod cli;
mod core;
mod parser;
mod plotting;
mod io;
mod utils;

fn main() {
    println!("Welcome to Rust-MATLAB!");

    cli::run_repl();

}