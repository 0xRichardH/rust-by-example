use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use super::{
    commands::BankAccountCommand, errors::BankAccountError, events::BankAccountEvent,
    services::BankAccountService,
};

#[derive(Serialize, Deserialize, Default)]
pub struct BankAccount {
    opened: bool,
    balance: f64,
}

#[async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Error = BankAccountError;
    type Services = BankAccountService;

    fn aggregate_type() -> String {
        String::from("account")
    }

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { .. } => self.opened = true,
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance
            }
            BankAccountEvent::CustomerWithdrewMoney { amount: _, balance } => {
                self.balance = balance
            }
            BankAccountEvent::CustomerWroteCheck {
                check_number: _,
                amount: _,
                balance,
            } => self.balance = balance,
        }
    }
}
