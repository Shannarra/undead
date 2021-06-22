mod utilities;
mod generator;
mod entities;
mod built_in;
mod error;

use utilities::read_lines;
use crate::error::{ErrorType, UndeadError};

fn perform_ritual(ritual_code: String, debug: bool) {
    let entities = generator::generate_all(&ritual_code);

    if let Err(e) = &entities {
        eprintln!("{:?}", e);
        std::process::exit(1);
    } else if debug {
        let entities = entities.unwrap();
        println!("Entity Names - Types:");
        for name in entities.keys().into_iter() {
            println!("{} :: {}", name, &entities.get(name).unwrap().entity_type());
        }
    }


}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let mut filename = "./tests/hello_world.zombie";
    let mut debug_entities = false;

    if args.len() > 1 && std::path::Path::new(&args[1]).exists() {
        filename = &args[1];
    }
    if args.contains(&"--debug_entities".to_string()) {
        debug_entities = true;
        println!("Entity debug active!");
    }

    if let Ok(text) = read_lines(filename) {
        let mut zombie_code = String::new();
        for line in text {
            let line = &line?;
            zombie_code += &("\n".to_string() + line.trim());
        }

        perform_ritual(zombie_code, debug_entities);
    } else {
        panic!("Reading file \"{}\" failed.", filename);
    }

    Ok(())
}
