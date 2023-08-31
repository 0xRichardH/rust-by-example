use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct BankAccountError(pub String);

impl Display for BankAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BankAccountError {}

impl From<&str> for BankAccountError {
    fn from(value: &str) -> Self {
        BankAccountError(value.to_string())
    }
}
