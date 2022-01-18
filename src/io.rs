use std::fs;
use std::fs::File;
use std::io::Write;

pub fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("something went wrong:(")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn write_file(filename: &str, s: &String) {
    let mut output = File::create(filename).unwrap();
    output.write(s.as_ref()).unwrap();
}