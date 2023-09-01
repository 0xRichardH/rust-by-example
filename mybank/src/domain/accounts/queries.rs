use async_trait::async_trait;
use cqrs_es::{persist::GenericQuery, EventEnvelope, Query, View};
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

use super::{aggregate::BankAccount, events::BankAccountEvent};

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<BankAccount> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
            println!("metadata => {:#?}", &event.metadata);
            println!("===============================================");
        }
    }
}

pub type AccountQuery = GenericQuery<
    PostgresViewRepository<BankAccountView, BankAccount>,
    BankAccountView,
    BankAccount,
>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BankAccountView {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
    ledger: Vec<LedgerEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerEntry {
    description: String,
    amount: f64,
}

impl LedgerEntry {
    fn new(description: &str, amount: f64) -> Self {
        Self {
            description: description.to_string(),
            amount,
        }
    }
}

impl View<BankAccount> for BankAccountView {
    fn update(&mut self, event: &EventEnvelope<BankAccount>) {
        match &event.payload {
            BankAccountEvent::AccountOpened { account_id } => {
                self.account_id = Some(account_id.clone());
            }

            BankAccountEvent::CustomerDepositedMoney { amount, balance } => {
                self.ledger.push(LedgerEntry::new("deposit", *amount));
                self.balance = *balance;
            }

            BankAccountEvent::CustomerWithdrewMoney { amount, balance } => {
                self.ledger.push(LedgerEntry::new("withdrawal", *amount));
                self.balance = *balance;
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod query_tests {
    use std::{collections::HashMap, vec};

    use anyhow::Result;
    use cqrs_es::{mem_store::MemStore, CqrsFramework};

    use crate::domain::accounts::{commands::BankAccountCommand, services::BankAccountService};

    use super::*;

    // run this test case by using `cargo test -- --nocapture`
    #[tokio::test]
    async fn test_event_store() -> Result<()> {
        let event_store = MemStore::<BankAccount>::default();
        let query = SimpleLoggingQuery {};
        let service = BankAccountService {};
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], service);

        let aggregate_id = "aggregate-instance-a";

        // open account
        let mut metadata = HashMap::new();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        cqrs.execute_with_metadata(
            aggregate_id,
            BankAccountCommand::OpenAccount {
                account_id: String::from("test_account_id"),
            },
            metadata,
        )
        .await?;

        // deposit
        cqrs.execute(
            aggregate_id,
            BankAccountCommand::DepositMoney { amount: 20.0 },
        )
        .await?;

        // withdraw
        cqrs.execute(
            aggregate_id,
            BankAccountCommand::WithdrawMoney { amount: 10.0 },
        )
        .await?;

        Ok(())
    }
}
