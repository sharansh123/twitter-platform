use async_channel::Sender;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

pub struct KafkaConsumer{
    pub consumer: StreamConsumer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEvent {
    pub post_id: i32,
    pub user_id: String
}

impl KafkaConsumer {
    pub fn new() -> Self {
        let brokers = std::env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS not found");
        let group_id = std::env::var("KAFKA_GROUP_ID").expect("KAFKA_GROUP_ID not found");
        let topic_id = std::env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC not found");
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", &group_id)
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Unable to connect to brokers");

        consumer.subscribe(&[&topic_id]).expect("Couldn't subscribe to topic");

        KafkaConsumer {
            consumer
        }
    }

    pub async fn receive(&self, tx: Sender<PostEvent>) {
        let mut stream = self.consumer.stream();

        while let Some(message) = stream.next().await {
            match message {
                Ok(result) => {
                    let payload = result.payload().unwrap();
                    let postEvent: PostEvent = serde_json::from_slice(payload).expect("Unable to parse. Wrong payload!");
                    println!("Received from Kafka: {:?}", postEvent);
                    tx.send(postEvent).await.expect("Channel is closed.");
                }
                Err(e) => eprintln!("Kafka error: {}", e)
            }
        }
    }
}