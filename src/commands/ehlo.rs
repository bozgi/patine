use crate::commands::command::Command;

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

        response.push_str(format!("250-{}\r\n", self.greeting).as_str());

        for (index, line) in &self.lines.iter().enumerate() {
            if index == self.lines.len() - 1 {
                response.push_str(format!("250 {}\r\n", line).as_str());
                break
            }
            response.push_str(format!("250-{}\r\n", line).as_str());
        }


    }
}