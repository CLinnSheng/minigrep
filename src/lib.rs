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

    for r in search(&args.target, &contents, args.ignore_case) {
        println!("{r}");
    }

    Ok(())
}

fn search<'a>(
    target: &str,
    contents: &'a str,
    case_sensitive: bool,
) -> impl Iterator<Item = &'a str> {
    // Not case sensitive/ignore case just simply change all to lowercase and compare
    let target = if !case_sensitive {
        target.to_lowercase()
    } else {
        target.to_string()
    };

    contents.lines().filter(move |line| {
        if !case_sensitive {
            line.to_lowercase().contains(&target)
        } else {
            line.contains(&target)
        }
    })
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
            search(target, contents, true).collect::<Vec<&str>>()
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
            search(target, contents, false).collect::<Vec<&str>>()
        );
    }
}
