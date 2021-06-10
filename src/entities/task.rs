use crate::built_in::{self, say};

#[derive(Clone)]
pub struct Task {
    pub exec_code: Vec<String>,
    pub active: bool
}

impl Task {
    pub fn new(exec_code: Vec<String>, active: bool) -> Self { Self { exec_code, active } }

    pub fn execute(&self) -> bool {
        for task_line in self.exec_code[1..].iter() {
            let task_tokens = task_line.split(" ").collect::<Vec<&str>>();
            let command= task_tokens[0];
            let mut args :String = String::new();
            task_tokens[1..].iter().for_each(|x| { args += x; args += " " });

            args = args.trim_end().to_string();

            if command == "say" {
                if args.starts_with("\"") && args.ends_with("\"") {
                    let without_quotes = &args[1..args.len() - 1];
                    return say(without_quotes.to_string());
                } else {
                    panic!("Called \"say\" on a string not enclosed in double quotes.")
                }
            }


        }

        true
    }
}