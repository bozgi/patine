#[derive(Debug, Eq, PartialEq)]
pub enum SmtpCommand {
    Helo(String),
    Ehlo(String),
    Mail(String),
    Rcpt(String),
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
            SmtpCommand::Mail("Mail from".to_string())
        } else if string.starts_with("rcpt") {
            SmtpCommand::Rcpt("".to_string())
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

    pub fn name(&self) -> Option<&'static str> {
        match self {
            SmtpCommand::Helo(_) => Some("helo"),
            SmtpCommand::Ehlo(_) => Some("ehlo"),
            SmtpCommand::Mail(_) => Some("mail"),
            SmtpCommand::Rcpt(_) => Some("rcpt"),
            SmtpCommand::Data(_) => Some("data"),
            SmtpCommand::Rset => Some("rset"),
            SmtpCommand::Noop => Some("noop"),
            SmtpCommand::Quit => Some("quit"),
            SmtpCommand::Vrfy(_) => Some("vrfy"),
            SmtpCommand::Unknown => None,
        }
    }
}