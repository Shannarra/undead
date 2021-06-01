use crate::entities::{
    self,
    Entity,
    Zombie
};
use std::ops::Deref;

//#[allow(unused_variables)]
pub fn generate_all(zombie_code: &String) {
    let depth :u32 = 0;
    let mut next_summon = false;
    let mut entity: Option<Box<dyn Entity>> = None; //Box::<dyn Entity>::new();

    let lines: Vec<&str> = zombie_code.split("\n").collect::<Vec<&str>>();

    for line in 0..lines.len() {
        if !next_summon && lines[line].contains("is a") || lines[line].contains("is an") { // entity definition
            let contents = lines[line].split(" ").collect::<Vec<&str>>();

            //create entity
            if contents[contents.len() - 1].to_lowercase() == "zombie" {
                entity = Some(Box::new(Zombie::new(&contents[0])));
                next_summon = true;
            }
            // TODO: all other entities - ghost, vampire, demon, djinn

            continue;
        }

        if next_summon { // require "summon" on next line
            next_summon = false;

            if lines[line].to_lowercase() != "summon" {
                panic!("Expected \"summon\" after entity definition on line {}. \
                \nHint: use \"summon\" on line {} instead of \"{}\"", line - 1, line, lines[line]);
            }

            // if the entity has been summoned then print it out
            if let Some(e) = &entity {
                e.print_entity_data();
            }
        }
    }
}