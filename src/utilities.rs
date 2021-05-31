use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path> {
    let _file = File::open(file)?;
    Ok(io::BufReader::new(_file).lines())
}