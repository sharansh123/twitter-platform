mod database;

use std::sync::Arc;
use async_channel::Receiver;
use axum::Router;
use axum::routing::{get, post};
use rdkafka::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use tokio::net::TcpListener;
use crate::database::DB::DB;
use crate::database::KafkaConfig::{KafkaConsumer, PostEvent};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_connection = DB::new().await;
    let db = Arc::new(db_connection);
    let (tx, rx) = async_channel::bounded::<PostEvent>(100);
    let consumer = KafkaConsumer::new();
    executor(&db, rx);
    consumer.receive(tx).await;
    let app: Router = Router::new().route("/health", get(async || "Up and Running".to_string()))
        .with_state(db);
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    println!("Task Creator Started!");

    axum::serve(listener, app).await.unwrap();
}

fn executor(db: &Arc<DB>, rx: Receiver<PostEvent>) {
    for i in 0..10 {
        let rec = rx.clone();
        let db_c = Arc::clone(db);
        tokio::spawn(async move {
            while let Ok(msg) = rec.recv().await {
                println!("Worker Thread receive this message: {:?}", msg);
                db_c.split_into_tasks(msg).await;
            }
        });
    }
}

