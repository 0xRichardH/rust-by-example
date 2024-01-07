pub trait State<T> {
    fn to_state(&self) -> T;
}

#[derive(Debug, PartialEq)]
pub enum TransactionStatus {
    Pending {
        transaction_id: String,
        amount: u128,
    },
    Approved {
        transaction_id: String,
        amount: u128,
    },
    Declined {
        transaction_id: String,
        amount: u128,
    },
    Error {
        transaction_id: String,
    },
}

impl<S> From<S> for TransactionStatus
where
    S: State<TransactionStatus>,
{
    fn from(state: S) -> Self {
        state.to_state()
    }
}

#[derive(Clone)]
pub struct Pending {
    transaction_id: String,
    amount: u128,
}

impl Pending {
    pub fn new(transaction_id: String, amount: u128) -> Self {
        Self {
            transaction_id,
            amount,
        }
    }
}

impl State<TransactionStatus> for Pending {
    fn to_state(&self) -> TransactionStatus {
        TransactionStatus::Pending {
            transaction_id: self.transaction_id.clone(),
            amount: self.amount,
        }
    }
}

#[derive(Clone)]
pub struct Approved {
    transaction_id: String,
    amount: u128,
}

impl From<Pending> for Approved {
    fn from(state: Pending) -> Self {
        Self {
            transaction_id: state.transaction_id,
            amount: state.amount,
        }
    }
}

impl State<TransactionStatus> for Approved {
    fn to_state(&self) -> TransactionStatus {
        TransactionStatus::Approved {
            transaction_id: self.transaction_id.clone(),
            amount: self.amount,
        }
    }
}

#[derive(Clone)]
pub struct Declined {
    transaction_id: String,
    amount: u128,
}

impl From<Pending> for Declined {
    fn from(state: Pending) -> Self {
        Self {
            transaction_id: state.transaction_id,
            amount: state.amount,
        }
    }
}

impl State<TransactionStatus> for Declined {
    fn to_state(&self) -> TransactionStatus {
        TransactionStatus::Declined {
            transaction_id: self.transaction_id.clone(),
            amount: self.amount,
        }
    }
}

#[derive(Clone)]
pub struct Error {
    transaction_id: String,
}

impl From<Pending> for Error {
    fn from(state: Pending) -> Self {
        Self {
            transaction_id: state.transaction_id,
        }
    }
}

impl State<TransactionStatus> for Error {
    fn to_state(&self) -> TransactionStatus {
        TransactionStatus::Error {
            transaction_id: self.transaction_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_status() {
        let pending = Pending::new("123".to_string(), 100);
        assert_eq!(
            TransactionStatus::Pending {
                transaction_id: "123".to_string(),
                amount: 100
            },
            pending.to_state()
        );

        let approved = Approved::from(pending.clone());
        assert_eq!(
            TransactionStatus::Approved {
                transaction_id: "123".to_string(),
                amount: 100
            },
            approved.into()
        );

        let declined = Declined::from(pending.clone());
        assert_eq!(
            TransactionStatus::Declined {
                transaction_id: "123".to_string(),
                amount: 100
            },
            declined.into()
        );

        let error = Error::from(pending.clone());
        assert_eq!(
            TransactionStatus::Error {
                transaction_id: "123".to_string(),
            },
            error.into()
        );
    }
}
