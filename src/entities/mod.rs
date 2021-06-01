pub mod zombie;
mod task;

use task::Task;
pub use zombie::Zombie;

/// A trait representing what
/// an Entity means in the Zombie
/// language.
pub trait Entity {
    fn name(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn is_active(&self) -> bool;
    fn tasks_count(&self) -> i32;
    fn perform_tasks(&self);

    /*dev ops*/

    fn print_entity_data(&self);
    fn entity_scope(&self) -> Option<(usize, usize)>;
}

/// A necromancer is what executes
/// an `Entity`'s tasks
pub struct Necromancer {
    pub tasks: Vec<Task>,
    execution_layer: i32,
    done: bool
}

impl Necromancer {
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(1000),
            execution_layer: 0,
            done: false
        }
    }

    pub fn execute_task(&self, task_nr: i32) {
        let text = &self.tasks[task_nr as usize].exec_code.split("\n").collect::<Vec<&str>>();

        for mut task_arg in text {
            if task_arg.starts_with("null") {
                task_arg = &&*task_arg.replace("null", "")
            }
        }
    }
}