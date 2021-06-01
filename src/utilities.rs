use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path> {
    let str = &file.as_ref().to_string_lossy();
    let split = str.split('.').collect::<Vec<&str>>();

    if split[split.len() - 1] != "zombie" {
        panic!("Please, provide a .zombie file. \n\".{}\" file given.", split[split.len() - 1])
    }


    let _file = File::open(file)?;
    Ok(io::BufReader::new(_file).lines())
}