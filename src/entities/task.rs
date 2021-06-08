#[derive(Clone)]
pub struct Task {
    pub exec_code: String,
    pub active: bool
}

impl Task {
    pub fn new(exec_code: String, active: bool) -> Self { Self { exec_code, active } }
}