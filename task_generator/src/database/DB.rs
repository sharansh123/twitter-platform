use axum::http::StatusCode;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use crate::database::KafkaConfig::PostEvent;
use crate::database::MQConfig::MessageQueue;

pub struct DB{
    db: sqlx::PgPool,
    queue: MessageQueue
}

#[derive(Debug)]
pub struct Follower {
    follower_id: String,
    offset_val:  i32
}

#[derive(Debug, Serialize)]
pub struct PostTask {
    follower_id: String,
    post_id:  i32
}

impl DB {

    pub async fn new() -> Self {

        let url = std::env::var("DATABASE_URL").expect("Database not found");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .expect("Failed to connect to DB");

        DB {
            db: pool,
            queue: MessageQueue::new().await
        }
    }

    pub async fn split_into_tasks(&self, event: PostEvent) {
        let user_id = event.user_id;
        let post_id : i32 = event.post_id;
        let mut offset_op = Some(0);
        while let Some(offset) = offset_op {
            let result = sqlx::query_as!(Follower,
                "select follower_id, offset_val from user_follow where followed_id = $1 and offset_val > $2 ORDER BY offset_val LIMIT 100",
            user_id, offset).fetch_all(&self.db)
                .await.map_err(|_| StatusCode::NOT_FOUND);

            match result {
                Ok(r) => {
                    println!("{:?}", r);
                    offset_op = r.last().map(|x| x.offset_val);
                    let task: Vec<PostTask> = r.iter().map(|x| PostTask{
                        follower_id: x.follower_id.clone(),
                        post_id
                    }).collect();
                    if !task.is_empty() {
                        println!("Sending to Message Queue");
                        let queue = std::env::var("RABBIT_MS_ADDR").expect("RABBIT_MS_ADDR not found");
                        let payload = serde_json::to_vec(&task).expect("Unable to serialize");
                        let result = self.queue.push(&queue, &payload).await;
                        match result {
                            Ok(_) => println!("Sent payload successfully"),
                            Err(_) => eprintln!("Unable to send payload")
                        }
                    }
                },
                Err(_) => break
            }

        }
    }
}