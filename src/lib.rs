use std::{error::Error, fs};

pub fn run(target: String, file_path: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    for line in search(&target, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
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
