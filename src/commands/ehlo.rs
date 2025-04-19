use crate::commands::command::Command;

struct Ehlo {
    greeting: String,
    lines: Vec<String>,
}

impl Command for Ehlo {
    fn execute(&mut self) -> Result<String, String> {
        if self.lines.is_empty() {
            return Ok(format!("250 {}", self.greeting));
        }

        let mut response = format!("250-{}\r\n", self.greeting);
        for line in self.lines.iter().take(self.lines.len() - 1) {
            response.push_str(&format!("250-{}\r\n", line));
        }

        if let Some(last) = self.lines.last() {
            response.push_str(&format!("250 {}", last));
        }

        Ok(response)
    }
}