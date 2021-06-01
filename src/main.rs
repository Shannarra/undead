mod utilities;
mod generator;
mod entities;

use utilities::read_lines;


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

        generator::generate_all(&zombie_code);
    } else {
        panic!("Reading file \"{}\" failed.", filename);
    }

    Ok(())
}
