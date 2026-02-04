use lapin::{Connection, ConnectionProperties};
use tokio_executor_trait::Tokio as TokioExecutor;
use tokio_reactor_trait::Tokio as TokioReactor;

#[allow(unused)]
pub async fn connect() -> anyhow::Result<Connection> {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".into());

    let props = ConnectionProperties::default()
        .with_executor(TokioExecutor::current())
        .with_reactor(TokioReactor::current());

    Ok(Connection::connect(&addr, props).await?)
}
