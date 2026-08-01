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

    let res = if args.ignore_case {
        search_case_insensitive(&args.target, &contents)
    } else {
        search_case_sensitive(&args.target, &contents)
    };

    for r in res {
        println!("{r}");
    }

    Ok(())
}

fn search_case_sensitive<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
    let mut output = Vec::new();

    for line in contents.lines() {
        if line.contains(target) {
            output.push(line);
        }
    }

    output
}

fn search_case_insensitive<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
    // Simply make every case to lowercase and match with lowercase
    let target = target.to_lowercase();
    let mut output = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&target) {
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

        assert_eq!(
            vec!["safe, fast, productive"],
            search_case_sensitive(target, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let target = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust Me.";

        assert_eq!(
            vec!["Rust:", "Trust Me."],
            search_case_insensitive(target, contents)
        );
    }
}
