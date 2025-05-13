pub enum SmtpResponse {
    SingleLine(u16, String),
    Multiline(u16, Vec<String>)
}