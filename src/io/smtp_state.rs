pub enum SmtpState {
    Connected,
    Greeted,
    Mailing,
    Addressing,
    Sending,
    Finished,
}