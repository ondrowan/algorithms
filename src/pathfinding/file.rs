use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_map(path: &str) -> Vec<String> {
    if let Result::Ok(file) = File::open(path) {
        BufReader::new(file).lines().map(|l| l.unwrap()).collect()
    } else {
        panic!("File not found!");
    }
}
