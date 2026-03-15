use std::time::Duration;
use rdkafka::{producer, ClientConfig};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

pub struct KafkaProducer{
    producer: FutureProducer
}

#[derive(Serialize)]
pub struct Message {
    pub post_id: i32,
    pub user_id: String
}


pub fn new() -> KafkaProducer {

    let brokers = std::env::var("KAFKA_BROKERS").expect("Brokers not found");
    let producer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer Creation Error");

    KafkaProducer{
        producer
    }
}

impl KafkaProducer{

    pub async fn send(&self, topic: String, payload: Message) {
        let payload_ser = serde_json::to_vec(&payload).expect("Message unable to serialize");
        if !payload.user_id.is_empty() {
            let record = FutureRecord::to(&topic)
                .key(&payload.user_id)
                .payload(&payload_ser);
            match self.producer.send(record, Duration::from_secs(0)).await {
                Ok((partition, offset)) => println!("Sent to partition {} with offset {}", partition, offset),
                Err((e, _)) => eprintln!("Error: {}", e)
            }
        }
    }
}