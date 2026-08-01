use std::process;

use clap::Parser;

use crate::lib::Args;

mod lib;

fn main() {
    let args = Args::parse();

    println!("Searching for {}", args.target);
    println!("In file {}", args.file_path);

    if let Err(e) = lib::run(args) {
        println!("Error {e}");
        process::exit(1);
    }
}
