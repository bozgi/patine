use std::fmt::Display;

#[derive(Debug)]
pub enum SmtpCommand {
    Helo(String),
    Ehlo(String),
    MailFrom(String),
    RcptTo(String),
    Rset,
    Noop,
    Quit,
    Vrfy(String),
    Unknown
}

impl SmtpCommand {
    pub fn from(string: String) -> Self {
        let string = string.to_lowercase();

        if string.starts_with("ehlo") || string.starts_with("helo") {
            SmtpCommand::Ehlo("Hellow!".to_string())
        } else if string.starts_with("mail from") {
            SmtpCommand::MailFrom("Mail from".to_string())
        } else if string.starts_with("quit") {
            SmtpCommand::Quit
        } else {
            SmtpCommand::Unknown
        }
    }
}