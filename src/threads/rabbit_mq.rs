use anyhow::Result;
use lapin::{Connection, ConnectionProperties};

pub async fn connect() -> Result<Connection> {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".into());

    let props = ConnectionProperties::default()
        .with_executor(lapin::executor::TokioExecutor::current())
        .with_reactor(lapin::reactor::TokioReactor);

    Ok(Connection::connect(&addr, props).await?)
}
