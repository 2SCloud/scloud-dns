use anyhow::Result;
use futures_util::StreamExt;
use lapin::{
    options::*,
    types::FieldTable,
    BasicProperties, Connection
};
use crate::threads::task::ScloudWorkerTask;

#[allow(unused)]
pub async fn send_task_and_wait(
    conn: &Connection,
    task: &ScloudWorkerTask,
) -> Result<serde_json::Value> {
    let channel = conn.create_channel().await?;

    let reply = channel
        .queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let correlation_id = task.task_id.to_string();

    let payload = serde_json::to_vec(task)?;

    channel
        .basic_publish(
            "scloud.jobs",
            "worker.dns",
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default()
                .with_correlation_id(correlation_id.clone().into())
                .with_reply_to(reply.name().as_str().into())
                .with_delivery_mode(2),
        )
        .await?
        .await?;

    let mut consumer = channel
        .basic_consume(
            reply.name().as_str(),
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        if delivery.properties.correlation_id().as_ref().map(|c| c.as_str())
            == Some(correlation_id.as_str())
        {
            let response: serde_json::Value =
                serde_json::from_slice(&delivery.data)?;
            delivery.ack(BasicAckOptions::default()).await?;
            return Ok(response);
        }
        delivery.ack(BasicAckOptions::default()).await?;
    }

    anyhow::bail!("No response received")
}