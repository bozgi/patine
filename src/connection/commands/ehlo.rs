use crate::connection::commands::command::Command;

struct Ehlo {
    greeting: String,
    lines: Vec<String>,
}

impl Command for Ehlo {
    fn execute(&mut self) {
        let mut response: String = String::new();
        if self.lines.len() == 0 {
            response = format!("250 {}", self.greeting);
            return;
        }
    }
}