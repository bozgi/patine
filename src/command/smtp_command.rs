#[derive(Debug, Eq, PartialEq)]
pub enum SmtpCommand {
    Helo(String),
    Ehlo(String),
    Mail(String),
    Rcpt(String),
    Data,
    Rset,
    Noop,
    Quit,
    Vrfy(String),
    Starttls,
    Auth(String),
    Unknown,
    DataEnd(Vec<u8>), // not an SMTP command, but a data passing object for handling mail delivery
}

impl SmtpCommand {
    pub fn from(string: String) -> Self {
        let params = string.get(4..).unwrap_or("").trim().to_string();
        let string = string.to_lowercase();

        if string.starts_with("ehlo") {
            SmtpCommand::Ehlo(params)
        } else if string.starts_with("helo") {
            SmtpCommand::Helo(params)
        } else if string.starts_with("mail") {
            SmtpCommand::Mail(params)
        } else if string.starts_with("rcpt") {
            SmtpCommand::Rcpt(params)
        } else if string.starts_with("data") {
            SmtpCommand::Data
        } else if string.starts_with("rset") {
            SmtpCommand::Rset
        } else if string.starts_with("noop") {
            SmtpCommand::Noop
        } else if string.starts_with("quit") {
            SmtpCommand::Quit
        } else if string.starts_with("vrfy") {
            SmtpCommand::Vrfy(params)
        } else if string.starts_with("starttls") {
            SmtpCommand::Starttls
        } else if string.starts_with("auth") {
            SmtpCommand::Auth(params)
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
            SmtpCommand::Data => Some("data"),
            SmtpCommand::Rset => Some("rset"),
            SmtpCommand::Noop => Some("noop"),
            SmtpCommand::Quit => Some("quit"),
            SmtpCommand::Vrfy(_) => Some("vrfy"),
            SmtpCommand::Auth(_) => Some("auth"),
            SmtpCommand::Starttls => Some("starttls"),
            SmtpCommand::DataEnd(_) => Some("data_end"),
            _ => None,
        }
    }
}