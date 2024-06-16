use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(FromRow, Serialize, Deserialize)]
pub struct RecipeThumbnail {
    pub recipe_id: i32,
    pub thumbnail_path: Option<String>,
}

impl RecipeThumbnail {
    pub async fn insert_or_update(
        pool: &Pool<Postgres>,
        recipe_id: i32,
        thumbnail_path: String,
    ) -> Result<(), anyhow::Error> {
        let rec = sqlx::query(
            r#"INSERT INTO recipe_thumbnails(thumbnail_path, recipe_id) 
            VALUES ($1, $2) ON CONFLICT (recipe_id) 
            DO UPDATE SET thumbnail_path = excluded.thumbnail_path, recipe_id = excluded.recipe_id"#,
        )
        .bind(thumbnail_path)
        .bind(recipe_id)
        .execute(pool)
        .await;

        if let Err(e) = rec {
            let err = e.to_string();
            return Err(anyhow!("Failed to insert thumbnail for recipe: {}", err));
        }

        Ok(())
    }
}
