use crate::entities::Entity;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Zombie<'a> {
    name: &'a str
}

impl<'a> Zombie<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {name}
    }
}

impl<'a> Entity for Zombie<'a> {
    fn name(&self) -> &'a str { self.name }

    fn entity_type(&self) -> &'a str { "zombie" }

    fn is_active(&self) -> bool {
        todo!()
    }

    fn tasks_count(&self) -> i32 {
        0
    }

    fn perform_tasks(&self) {
        if self.tasks_count() >= 1 {
            todo!()
        } else {
            panic!("No tasks for current Zombie named \"{}\"", self.name);
        }
    }

    fn print_entity_data(&self) {
        println!("{}", &self);
    }
}

impl std::fmt::Display for Zombie<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This is a {} with the name {}. {} has {} tasks before disappearing again.",
               "zombie",
                &self.name,
                &self.name,
                &self.tasks_count()
        )
    }
}