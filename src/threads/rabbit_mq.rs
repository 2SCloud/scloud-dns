use anyhow::Result;
use lapin::{Connection, ConnectionProperties};

pub async fn connect() -> Result<Connection> {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".into());

    let props = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio::current());

    Ok(Connection::connect(&addr, props).await?)
}
