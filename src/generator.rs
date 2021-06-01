use crate::entities::{
    self,
    Entity,
    Zombie
};
use std::ops::Deref;


pub fn generate_all(zombie_code: &String) {

    if zombie_code.matches("summon").count() != zombie_code.matches("bind").count() {
        panic!("All summoned entities must be bound after execution!");
    }

    let mut entities_list: Vec<String> = Vec::new();

    let depth :u32 = 0;

    let mut bounds: Vec<usize> = Vec::new();

    let mut next_summon = false;
    let mut entity: Option<Box<dyn Entity>> = None; //Box::<dyn Entity>::new();

    let lines: Vec<&str> = zombie_code.split("\n").collect::<Vec<&str>>();

    set_entities_bounds(&lines, &mut bounds);

    for line in 0..lines.len() {

        if !next_summon && lines[line].contains("is a") || lines[line].contains("is an") { // entity definition
            let contents = lines[line].split(" ").collect::<Vec<&str>>();

            // Entities' names MUST be unique
            if entities_list.contains(&contents[0].to_string()) {
                panic!("Entity with name \"{}\" already declared.", &contents[0]);
            }

            // Entity creation is coupled with it's definition.
            // This means that an entity with invalid scope could (theoretically) exist,
            // but the interpreter will panic! at the next line and dispose of it.
            if contents[contents.len() - 1] == "Zombie" {
                entity = Some(Box::new(
                                Zombie::with_scope(&contents[0],
                                           Some((line, bounds.pop().unwrap())))
                ));
                next_summon = true;
                entities_list.push(contents[0].to_string());
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

fn set_entities_bounds<'a>(text_lines: &Vec<&'a str>, bounds: &mut Vec<usize>) {
    let mut bind_search= text_lines.to_vec();

    let mut line_number = text_lines.len() - 1;

    bind_search.reverse();
    for line in bind_search {
        if line == "bind" {
            bounds.push(line_number);
        }
        if line_number > 0 {
            line_number -= 1;
        }
    }

    bounds.rotate_left(1)
}