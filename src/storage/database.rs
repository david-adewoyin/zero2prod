use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Debug)]
pub struct Storage {
    con: PgPool,
}

impl Storage {
    pub async fn new(connection_string: &str) -> Result<Self, &str> {
        let connection = PgPool::connect(connection_string).await.map_err(|_| "ss")?;
        Ok(Storage { con: connection })
    }
    #[tracing::instrument(name="Saving new subscriber details into database"
        fields( request_id = %Uuid::new_v4(),subscriber_email = %email, subscriber_name = %name))]
    pub async fn insert_subscriber(&self, name: &str, email: &str) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO subscriptions (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(email)
        .bind(Utc::now())
        .execute(&self.con)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query :{:?}", e);
            e.to_string()
        })?;
        Ok(())
    }
}
