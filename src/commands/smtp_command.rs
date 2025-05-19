use std::fmt::Display;

#[derive(Debug)]
pub enum SmtpCommand {
    Helo(String),
    Ehlo(String),
    MailFrom(String),
    RcptTo(String),
    Data(String),
    Rset,
    Noop,
    Quit,
    Vrfy(String),
    Unknown
}

impl SmtpCommand {
    pub fn from(string: String) -> Self {
        let string = string.to_lowercase();

        if string.starts_with("ehlo") {
            SmtpCommand::Ehlo("Hellow!".to_string())
        } else if string.starts_with("helo") {
            SmtpCommand::Helo("Hellow!".to_string())
        } else if string.starts_with("mail") {
            SmtpCommand::MailFrom("Mail from".to_string())
        } else if string.starts_with("rcpt") {
            SmtpCommand::RcptTo("".to_string())
        } else if string.starts_with("data") {
            SmtpCommand::Data("".to_string())
        } else if string.starts_with("rset") {
            SmtpCommand::Rset
        } else if string.starts_with("noop") {
            SmtpCommand::Noop
        } else if string.starts_with("quit") {
            SmtpCommand::Quit
        } else if string.starts_with("vrfy") {
            SmtpCommand::Vrfy("".to_string())
        } else {
            SmtpCommand::Unknown
        }
    }
}