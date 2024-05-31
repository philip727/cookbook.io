use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{pool, prelude::FromRow, types::chrono, Pool, Postgres};

use crate::helpers::is_alnum_whitespace;

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
    ) -> Result<(), anyhow::Error> {
        if !is_alnum_whitespace(&title) {
            return Err(anyhow::Error::msg(format!(
                "Failed to insert recipe step as the title isn't alphanumerical: {}",
                &title
            )));
        }

        if !is_alnum_whitespace(&description) {
            return Err(anyhow::Error::msg(format!(
                "Failed to insert recipe step as the description isn't alphanumerical: {}",
                &description
            )));
        }

        let rec = sqlx::query(
            r#"
            INSERT INTO recipes (title, description, user_id)
            VALUES ( $1, $2, $3 )"#,
        )
        .bind(title)
        .bind(description)
        .bind(user_id)
        .fetch_all(pool)
        .await;

        // Returns failed insert with message
        if let Err(e) = rec {
            let err = e.to_string();
            return Err(e).context(format!("Failed to insert recipe: {}", err));
        }

        Ok(())
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
        steps: Vec<(String, i32)>,
    ) -> Result<(), anyhow::Error> {
        for step in steps.iter() {
            let desc = &step.0;
            let order = step.1;

            if !is_alnum_whitespace(desc) {
                return Err(anyhow::Error::msg(format!(
                    "Failed to insert recipe step as it isn't alphanumerical: {}",
                    desc
                )));
            }

            let rec = sqlx::query(
                r#"
                INSERT INTO recipe_steps (recipe_id, description, step_order)
                VALUES ( $1, $2, $3 )"#,
            )
            .bind(recipe_id)
            .bind(desc)
            .bind(order)
            .fetch_one(pool)
            .await;

            // Returns failed insert with message
            if let Err(e) = rec {
                let err = e.to_string();
                return Err(e).context(format!("Failed to insert recipe step: {}", err));
            }
        }

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
}
