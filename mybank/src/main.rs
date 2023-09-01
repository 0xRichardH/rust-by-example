use std::collections::HashMap;

use anyhow::{Context, Result};
use cqrs_es::{persist::PersistedEventStore, CqrsFramework, Query};
use domain::accounts::{
    aggregate::BankAccount, commands::BankAccountCommand, queries::SimpleLoggingQuery,
    services::BankAccountService,
};
use postgres_es::{default_postgress_pool, PostgresEventRepository};
use sqlx::{Pool, Postgres};

mod domain;

#[tokio::main]
async fn main() -> Result<()> {
    let db_pool = create_db_pool().await;

    // run migration
    run_migration(db_pool.clone()).await.context("migration:")?;

    // create event repo
    let event_repo = config_event_repo(db_pool.clone()).await;
    let event_store = PersistedEventStore::new_event_store(event_repo);

    let simple_logging_query = SimpleLoggingQuery {};
    let queries: Vec<Box<dyn Query<BankAccount>>> = vec![Box::new(simple_logging_query)];
    let service = BankAccountService {};
    let cqrs = CqrsFramework::new(event_store, queries, service);

    let aggregate_id = "test_aggregate_id_a";

    // open an account
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    cqrs.execute_with_metadata(
        aggregate_id,
        BankAccountCommand::OpenAccount {
            account_id: String::from("test_account_id"),
        },
        metadata,
    )
    .await
    .context("open account failed")?;

    // deposit
    cqrs.execute(
        aggregate_id,
        BankAccountCommand::DepositMoney { amount: 100.0 },
    )
    .await
    .context("deposit money failed")?;

    // deposit
    cqrs.execute(
        aggregate_id,
        BankAccountCommand::DepositMoney { amount: 101.0 },
    )
    .await
    .context("deposit money failed")?;

    // withdraw
    cqrs.execute(
        aggregate_id,
        BankAccountCommand::WithdrawMoney { amount: 150.0 },
    )
    .await
    .context("withdraw money failed")?;

    Ok(())
}

async fn create_db_pool() -> Pool<Postgres> {
    let connection_string = "postgres://postgres:password@localhost:5432/mybank";
    default_postgress_pool(connection_string).await
}

async fn run_migration(pool: Pool<Postgres>) -> Result<()> {
    let mut conn = pool
        .acquire()
        .await
        .context("failed to acquire db connection")?;
    sqlx::migrate!()
        .run(&mut conn)
        .await
        .context("failed to run migration")?;
    Ok(())
}

async fn config_event_repo(pool: Pool<Postgres>) -> PostgresEventRepository {
    PostgresEventRepository::new(pool).with_tables("account_events", "account_snapshots")
}
