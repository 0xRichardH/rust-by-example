use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};

use super::aggregate::BankAccount;

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<BankAccount> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}

#[cfg(test)]
mod query_tests {
    use std::vec;

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
