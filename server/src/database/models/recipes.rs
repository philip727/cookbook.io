use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono, Pool, Postgres};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub date_created: chrono::DateTime<chrono::Utc>,
}

impl Recipe {
    pub async fn get_with_pagination(pool: &Pool<Postgres>, offset: u32, limit: u32) -> Result<Vec<Recipe>, anyhow::Error> {
        let rows = sqlx::query_as::<_, Recipe>(r#"SELECT * FROM recipes ORDER by id LIMIT $1 OFFSET $2"#)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }
}
