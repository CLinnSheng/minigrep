use clap::Parser;
use std::process;

mod lib;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Mini version of grep")]
pub(crate) struct Args {
    target: String,
    file_path: String,
}

fn main() {
    let args = Args::parse();

    println!("Searching for {}", args.target);
    println!("In file {}", args.file_path);

    if let Err(e) = lib::run(args.target, args.file_path) {
        println!("Error {e}");
        process::exit(1);
    }
}
