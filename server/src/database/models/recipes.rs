use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{pool, prelude::FromRow, types::chrono, Pool, Postgres};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub date_created: chrono::DateTime<chrono::Utc>,
}

impl Recipe {
    pub async fn get_with_pagination(
        pool: &Pool<Postgres>,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Recipe>, anyhow::Error> {
        let rows =
            sqlx::query_as::<_, Recipe>(r#"SELECT * FROM recipes ORDER BY id LIMIT $1 OFFSET $2"#)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(pool)
                .await?;

        Ok(rows)
    }

    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        id: i32,
    ) -> Result<Option<Recipe>, anyhow::Error> {
        let row = sqlx::query_as::<_, Recipe>(r#"SELECT * FROM recipes WHERE id = $1"#)
            .bind(id)
            .fetch_one(pool)
            .await;

        if let Err(e) = row {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(e).context(format!("Failed to find user with id: {}", id));
        }

        Ok(Some(row.unwrap()))
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct RecipeSteps {
    pub id: i32,
    pub recipe_id: i32,
    pub description: String,
    // The order of which it is in the recipe
    pub step_order: i32,
}

impl RecipeSteps {
    pub async fn get_recipe_steps(
        pool: &Pool<Postgres>,
        id: i32,
    ) -> Result<Option<Vec<RecipeSteps>>, anyhow::Error> {
        let row = sqlx::query_as::<_, RecipeSteps>(
            r#"SELECT * FROM recipe_steps WHERE recipe_id = $1 ORDER BY step_order"#,
        )
        .bind(id)
        .fetch_all(pool)
        .await;

        if let Err(e) = row {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(e).context(format!("Failed to find user with id: {}", id));
        }

        Ok(Some(row.unwrap()))
    }
}
