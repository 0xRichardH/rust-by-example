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
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            BankAccountCommand::DepositMoney { amount } => {
                let balance = self.balance + amount;
                let event = BankAccountEvent::CustomerDepositedMoney { amount, balance };
                Ok(vec![event])
            }
            BankAccountCommand::WithdrawMoney { amount } => {
                if amount > self.balance {
                    return Err(BankAccountError(String::from("insufficent funds")));
                }

                let balance = self.balance - amount;
                let event = BankAccountEvent::CustomerWithdrewMoney { amount, balance };
                Ok(vec![event])
            }
            _ => Ok(vec![]),
        }
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

#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use cqrs_es::test::TestFramework;

    type AccountTestFramework = TestFramework<BankAccount>;

    #[test]
    fn it_should_deposit_money() {
        let expected = BankAccountEvent::CustomerDepositedMoney {
            amount: 23.0,
            balance: 23.0,
        };

        AccountTestFramework::with(BankAccountService)
            .given_no_previous_events()
            .when(BankAccountCommand::DepositMoney { amount: 23.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn it_should_deposit_money_with_balance() {
        let previous_event = BankAccountEvent::CustomerDepositedMoney {
            amount: 23.0,
            balance: 23.0,
        };
        let expected_event = BankAccountEvent::CustomerDepositedMoney {
            amount: 30.0,
            balance: 53.0,
        };

        AccountTestFramework::with(BankAccountService)
            .given(vec![previous_event])
            .when(BankAccountCommand::DepositMoney { amount: 30.0 })
            .then_expect_events(vec![expected_event]);
    }

    #[test]
    fn it_should_withdraw_money() {
        let previous_event = BankAccountEvent::CustomerDepositedMoney {
            amount: 10.0,
            balance: 10.0,
        };
        let expected_event = BankAccountEvent::CustomerWithdrewMoney {
            amount: 9.0,
            balance: 1.0,
        };

        AccountTestFramework::with(BankAccountService)
            .given(vec![previous_event])
            .when(BankAccountCommand::WithdrawMoney { amount: 9.0 })
            .then_expect_events(vec![expected_event]);
    }

    #[test]
    fn it_should_not_withdraw_money_because_of_insufficent_funds() {
        AccountTestFramework::with(BankAccountService)
            .given_no_previous_events()
            .when(BankAccountCommand::WithdrawMoney { amount: 10.0 })
            .then_expect_error(BankAccountError(String::from("insufficent funds")));
    }
}
