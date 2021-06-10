use crate::entities::{Entity, Task};
use std::fmt::Display;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Zombie<'a> {
    pub name: &'a str,
    pub tasks: Option<VecDeque<Task>>,
    active: bool,
    scope: Option<(usize, usize)>,
}

impl<'a> Zombie<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, tasks: None, active: false, scope: None }
    }

    pub fn with_scope(name: &'a str, scope: Option<(usize, usize)>) -> Self {
        Self {name, tasks: None, active: false, scope}
    }

    fn entity_will_live_for(&self) -> String {
        if let Some(scope) = self.scope {
            return format!("It will live from line {} to line {}.", scope.0, scope.1)
        }
        String::from("")
    }

    fn entity_active_state(&self) -> String {
        if self.active {
            return "active".to_string()
        }
        "inactive".to_string()
    }
}

impl<'a> Entity for Zombie<'a> {
    fn name(&self) -> &'a str { self.name }

    fn entity_type(&self) -> &'a str { "zombie" }

    fn is_active(&self) -> bool {
        self.active
    }

    fn tasks_count(&self) -> usize {
        if let Some(tasks) = &self.tasks {
            return tasks.len()
        }
        0
    }

    fn perform_tasks(&mut self) {
        if self.tasks_count() >= 1 {
            if let Some(queue) = &self.tasks {
                for task in queue {
                    if !task.execute() {
                        panic!("{} named \"{}\" could not execute task with code: {:?}", &self.entity_type(), &self.name, task.exec_code)
                    }
                }
                self.tasks.as_mut().unwrap().clear();
            } else {
                panic!("Task queue empty with tasks count higher than 0!");
            }
        } else {
            panic!("Called perform tasks for current {} named \"{}\" with no tasks", &self.entity_type(), &self.name);
        }
    }

    fn set_tasks(&mut self, tasks: VecDeque<Task>) { self.tasks = Some(tasks); }

    fn toggle_active(&mut self) {
        self.active = !self.active;
    }

    fn print_entity_data(&self) { println!("{}", &self); }

    fn entity_scope(&self) -> Option<(usize, usize)> { self.scope }
}

impl std::fmt::Display for Zombie<'_> {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This is an {} {} with the name {}. {} has {} tasks left before disappearing again. {}",
               &self.entity_active_state(),
               "zombie",
               &self.name,
               &self.name,
               &self.tasks_count(),
               &self.entity_will_live_for()
        )
    }
}