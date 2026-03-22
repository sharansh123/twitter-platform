use lapin::{Connection, ConnectionProperties, Error};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};

pub struct MessageQueue{
    connection: Connection
}


impl MessageQueue {

    pub async fn new() -> MessageQueue {
        let addr = std::env::var("RABBIT_MS_ADDR").expect("RABBIT_MS_ADDR not found");
        let options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        let connection= Connection::connect(&addr, options).await.expect("Unable to connect message queue");
        MessageQueue {
            connection
        }
    }


    pub async fn push(&self, queue: &str, payload: &[u8]) -> Result<(),Error>{

        let channel = self.connection.create_channel().await?;

        channel.queue_declare(queue, QueueDeclareOptions::default(), Default::default()).await?;

        channel.basic_publish("",
        queue,
        BasicPublishOptions::default(),
        payload,
        Default::default()
        ).await?;

        Ok(())
    }
}