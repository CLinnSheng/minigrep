use clap::Parser;
use std::{error::Error, fs};

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Mini version of grep")]
pub struct Args {
    pub target: String,
    pub file_path: String,

    #[arg(short, long)]
    pub ignore_case: bool,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    for line in search(&args.target, &contents, args.ignore_case) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(target: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut output = Vec::new();

    for line in contents.lines() {
        if line.contains(target) {
            output.push(line);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let target = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(target, contents));
    }

    #[test]
    fn case_insensitive() {
        let target = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust Me.";

        assert_eq!(vec!["Rust", "Trust Me."], search(target, contents));
    }
}
