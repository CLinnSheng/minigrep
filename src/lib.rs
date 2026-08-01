use std::{error::Error, fs};

pub fn run(target: String, file_path: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(())
}

fn search<'a>(target: &'a str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!()
}
