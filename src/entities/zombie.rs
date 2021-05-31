use crate::entities::Entity;

pub struct Zombie<'a> {
    name: &'a str
}

impl<'a> Zombie<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {name}
    }
}

impl<'a> Entity for Zombie<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn is_active(&self) -> bool {
        todo!()
    }

    fn tasks_count(&self) -> i32 {
        todo!()
    }

    fn perform_tasks(&self) {
        if self.tasks_count() >= 1 {
            todo!()
        } else {
            panic!("No tasks for current Zombie named \"{}\"", self.name);
        }
    }
}