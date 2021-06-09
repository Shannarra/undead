#[derive(Clone)]
pub struct Task {
    pub exec_code: Vec<String>,
    pub active: bool
}

impl Task {
    pub fn new(exec_code: Vec<String>, active: bool) -> Self { Self { exec_code, active } }

    pub fn execute(&self) -> bool {
        for task_line in self.exec_code[1..].iter() {
            println!("{}", task_line);
        }

        true
    }
}