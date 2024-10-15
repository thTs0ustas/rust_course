use std::fs::read_to_string;

use std::io::{Error, ErrorKind::NotFound};

pub fn write_to_file(file: &str, contents: &Vec<String>) -> Result<(), Error> {
    let contents = contents.join("\n");
    std::fs::write(file, contents)
}

pub fn read_log(file: &str) -> Result<Vec<String>, Error> {
    match read_to_string(file) {
        Ok(file_contents) => Ok(extract_errors(file_contents.as_str())),
        Err(_) => Err(Error::new(NotFound, "File not found")),
    }
}

fn extract_errors(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line| line.to_string())
        .collect()
}
