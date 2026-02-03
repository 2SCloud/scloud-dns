use lapin::{Connection, ConnectionProperties};

pub async fn connect() -> anyhow::Result<Connection> {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".into());

    Ok(Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await?)
}
