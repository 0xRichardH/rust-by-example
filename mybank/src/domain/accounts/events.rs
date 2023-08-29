use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let event_type = match self {
            BankAccountEvent::AccountOpened { .. } => "account_opened",
            BankAccountEvent::CustomerDepositedMoney { .. } => "customer_deposited_money",
            BankAccountEvent::CustomerWithdrewMoney { .. } => "customer_withdrew_money",
            BankAccountEvent::CustomerWroteCheck { .. } => "customer_wrote_check",
        };
        String::from(event_type)
    }

    fn event_version(&self) -> String {
        String::from("1.0")
    }
}
