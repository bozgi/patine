#[derive(Eq, PartialEq)]
pub enum TransactionType {
    SERVER, 
    SUBMISSION,
    CLIENT {
        exchange: Option<String>,
    }
}