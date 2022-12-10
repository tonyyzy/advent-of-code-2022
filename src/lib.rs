use std::{
    fs,
    io::{self, BufRead},
};

pub fn read_to_lines(path: &str) -> Vec<String> {
    let f = fs::File::open(path).unwrap();
    io::BufReader::new(f).lines().map(|a| a.unwrap()).collect()
}
