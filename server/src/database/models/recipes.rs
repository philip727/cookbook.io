use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{pool, prelude::FromRow, types::chrono, Pool, Postgres, Row};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub date_created: chrono::DateTime<chrono::Utc>,
}

impl Recipe {
    pub async fn insert(
        pool: &Pool<Postgres>,
        title: String,
        description: String,
        user_id: i32,
    ) -> Result<i32, anyhow::Error> {
        let rec = sqlx::query(
            r#"
            INSERT INTO recipes (title, description, user_id)
            VALUES ( $1, $2, $3 ) RETURNING id"#,
        )
        .bind(title)
        .bind(description)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let id: i32 = rec.try_get("id").context("Failed to id")?;

        Ok(id)
    }

    pub async fn exists(pool: &Pool<Postgres>, id: i32) -> bool {
        Recipe::get_by_id(pool, id).await.unwrap_or(None).is_some()
    }

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

            return Err(e).context(format!("Failed to find recipe with id: {}", id));
        }

        Ok(Some(row.unwrap()))
    }

    pub async fn delete(pool: &Pool<Postgres>, id: i32) -> Result<(), anyhow::Error> {
        let _ = sqlx::query(r#"DELETE FROM recipes WHERE id = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct RecipeStep {
    pub id: i32,
    pub recipe_id: i32,
    pub description: String,
    // The order of which it is in the recipe
    pub step_order: i32,
}

impl RecipeStep {
    pub async fn insert(
        pool: &Pool<Postgres>,
        recipe_id: i32,
        description: String,
        order: i32,
    ) -> Result<(), anyhow::Error> {
        let _ = sqlx::query(
            r#"
                INSERT INTO recipe_steps (recipe_id, description, step_order)
                VALUES ( $1, $2, $3 )"#,
        )
        .bind(recipe_id)
        .bind(description)
        .bind(order)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_recipe_steps(
        pool: &Pool<Postgres>,
        recipe_id: i32,
    ) -> Result<Option<Vec<RecipeStep>>, anyhow::Error> {
        let row = sqlx::query_as::<_, RecipeStep>(
            r#"SELECT * FROM recipe_steps WHERE recipe_id = $1 ORDER BY step_order"#,
        )
        .bind(recipe_id)
        .fetch_all(pool)
        .await;

        if let Err(e) = row {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(e).context(format!("Failed to find recipe step with id: {}", recipe_id));
        }

        Ok(Some(row.unwrap()))
    }

    pub async fn delete(pool: &Pool<Postgres>, recipe_id: i32) -> Result<(), anyhow::Error> {
        let _ = sqlx::query(r#"DELETE FROM recipe_steps WHERE recipe_id = $1"#)
            .bind(recipe_id)
            .execute(pool)
            .await
            .context("Failed to delete recipe steps")?;

        Ok(())
    }
}
