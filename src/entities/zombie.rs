use crate::entities::Entity;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Zombie<'a> {
    pub name: &'a str,
    scope: Option<(u32, u32)>
}

impl<'a> Zombie<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {name, scope: None }
    }

    pub fn with_scope(name: &'a str, scope: Option<(u32, u32)>) -> Self {
        Self {name, scope}
    }

    fn entity_will_live_for(&self) -> String {
        if let Some(scope) = self.scope {
            return format!("It will live from line {} to line {}.", scope.0, scope.1)
        }
        String::from("")
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
            panic!("No tasks for current {} named \"{}\"", self.entity_type(), self.name);
        }
    }

    fn print_entity_data(&self) {
        println!("{}", &self);
    }

    fn entity_scope(&self) -> Option<(u32, u32)> { self.scope }


}

impl std::fmt::Display for Zombie<'_> {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This is a {} with the name {}. {} has {} tasks before disappearing again. {}",
               "zombie",
                &self.name,
                &self.name,
                &self.tasks_count(),
                &self.entity_will_live_for()
        )
    }
}