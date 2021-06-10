use crate::entities::{
    self,
    Entity,
    Task,
    Zombie
};
use std::collections::{VecDeque, HashMap};
use std::borrow::BorrowMut;


pub fn generate_all(zombie_code: &String) {

    let mut all_entities :HashMap<&str, Box<dyn Entity>> = HashMap::new();

    if let Some(lines) = scope_check(&zombie_code) {
        let depth: u32 = 0;

        //let mut entities_list: Vec<String> = Vec::new();
        let mut bounds: Vec<usize> = Vec::new();
        let mut next_summon = false;

        //TODO: change box to something mutable
        //let mut entity: Option<Box<dyn Entity>> = None; //Box::<dyn Entity>::new();
        let mut current_entity_info :(&str, &str) = ("", "");
        let mut entity_bounds: (usize, usize) = (0,0);

        //let lines: Vec<&str> = zombie_code.split("\n").collect::<Vec<&str>>();

        set_entities_bounds(&lines, &mut bounds);

        for line in 0..lines.len() {
            if !next_summon && lines[line].contains("is a") || lines[line].contains("is an") { // entity definition
                let contents = lines[line].split(" ").collect::<Vec<&str>>();

                // Entities' names MUST be unique
                //if entities_list.contains(&contents[0].to_string()) {
                //
                //}

                for entity in &all_entities {
                    // collision. Entity with that name and type already exists.
                    if &entity.1.name() == &contents[0] &&
                        &entity.1.entity_type() == &contents[&contents.len() - 1].to_lowercase()
                    {
                        let scope = entity.1.entity_scope().unwrap();
                        panic!("Entity with name \"{}\" of type \"{}\" has been already declared in the scope: [{}, {}].",
                               &contents[0], &contents[&contents.len() - 1], scope.0, scope.1);
                    }
                }

                // Entity creation is coupled with it's definition.
                // This means that an entity with invalid scope could (theoretically) exist,
                // but the interpreter will panic! at the next line and dispose of it.

                if contents[contents.len() - 1] == "Zombie" {
                    entity_bounds = (line, bounds.pop().unwrap());

                    current_entity_info = (contents[contents.len() - 1], &contents[0]);

                    all_entities.insert(&contents[0], Box::new(
                        Zombie::with_scope(&contents[0],
                                           Some(entity_bounds))
                    ));

                    next_summon = true;
                    //entities_list.push(contents[0].to_string());
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

                if let Some(entity) = all_entities.get_mut(current_entity_info.1) {
                    entity.set_tasks(generate_tasks(&entity_bounds, &lines));
                }
            }

            if lines[line].starts_with("animate") || lines[line].starts_with("read_about") {
                let contents = lines[line].split(" ").collect::<Vec<&str>>();

                //TODO: DRY this check
                if contents.len() > 2 {
                    panic!("Animation expects only one argument.");
                } else if contents.len() < 2 {
                    panic!("Animation requires an Entity's name.");
                }
                if let Some(entity) = all_entities.get_mut(contents[1]) {
                    if lines[line].starts_with("animate") {
                        entity.perform_tasks();
                        entity.toggle_active();
                    } else if lines[line].starts_with("read_about") {
                        entity.print_entity_data();
                    }
                } else {
                    panic!("No entity to with name \"{}\" was found.", contents[1]);
                }
            }

            if lines[line].starts_with("banish") {
                let contents = lines[line].split(" ").collect::<Vec<&str>>();

                //TODO: DRY
                if contents.len() > 2 {
                    panic!("Animation expects only one argument.");
                } else if contents.len() < 2 {
                    panic!("Animation requires an Entity's name.");
                }

                if let Some(entity) = all_entities.get(contents[1]) {
                    all_entities.remove(contents[1]);
                } else {
                    panic!("No entity to banish with name \"{}\" was found.", contents[1]);
                }
            }
        }
    }
}

fn generate_tasks(range: &(usize, usize), text: &Vec<&str>) -> VecDeque<Task> {
    let mut task:Task;
    let mut current_task_code :Vec<String> = vec![];

    let mut q: VecDeque<Task> = VecDeque::new();

    for task_line in text[(range.0 + 2)..(range.1)].iter() {
        if task_line.starts_with("done") {
            task = Task::new(current_task_code, true);
            q.push_back(task);
            current_task_code = vec![];
        } else {
            current_task_code.push(task_line.to_string());
        }
    }

    q
}

fn scope_check(code: &String) -> Option<Vec<&str>> {
    //if zombie_code.matches("summon").count() + zombie_code.matches("task ").count() !=
    //    zombie_code.matches("bind").count() + zombie_code.matches("animate").count() {
    //    panic!("All summoned entities and tasks must be bound or animated after execution!");
    //}
    let lines: Vec<&str> = code.split("\n").collect::<Vec<&str>>();

    let mut summons_and_tasks = 0;
    let mut animations_and_bounds = 0;

    for line in &lines {
        if line.contains("summon") || line.contains("task") {
            if line.starts_with("say") || line.contains("\""){
                continue;
            } else {
                summons_and_tasks += 1;
            }
        }
        if line.contains("done") || line.contains("bind") {
            if line.starts_with("say") || line.contains("\""){
                continue;
            } else {
                animations_and_bounds += 1;
            }
        }
    }

    if animations_and_bounds != summons_and_tasks {
        panic!("Unbalanced summons/tasks with corresponding bind/done directives. \
        Please use \"bind\" or \"done\" when any entity or task are summoned.")
    }


    Some(lines)
}

fn set_entities_bounds<'a>(text_lines: &Vec<&'a str>, bounds: &mut Vec<usize>) {
    let mut bind_search= text_lines.to_vec();

    let mut line_number = text_lines.len() - 1;

    bind_search.reverse();
    for line in bind_search {
        if line == "bind" /* || line == "animate"  in order to use "animate" tasks must be on the queue as well */ {
            bounds.push(line_number);
        }
        if line_number > 0 {
            line_number -= 1;
        }
    }

    //bounds.rotate_left(1)
}