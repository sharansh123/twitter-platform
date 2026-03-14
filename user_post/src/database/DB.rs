use sqlx::postgres::PgPoolOptions;

pub struct DB {
    db: sqlx::PgPool
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
            db: pool
        }
    }
}