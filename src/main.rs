mod utilities;
mod generator;
mod entities;
mod built_in;
mod error;

use utilities::read_lines;
use crate::error::{ErrorType, UndeadError};

fn perform_ritual(ritual_code: String) {
    let entities = generator::generate_all(&ritual_code);

    if let Err(e) = &entities {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }

    for name in entities.unwrap().keys() {
        println!("{}", name);
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let mut filename = "./tests/hello_world.zombie";

    if args.len() > 1 && std::path::Path::new(&args[1]).exists() {
        filename = &args[1];
    }

    if let Ok(text) = read_lines(filename) {
        let mut zombie_code = String::new();
        for line in text {
            let line = &line?;
            zombie_code += &("\n".to_string() + line.trim());
        }

        perform_ritual(zombie_code);
    } else {
        panic!("Reading file \"{}\" failed.", filename);
    }

    Ok(())
}
