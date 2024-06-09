use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres, Row};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: i32,
    pub recipe_file_path: String,
    pub user_id: i32,
    pub date_created: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Poster {
    pub uid: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
// A recipe struct without user id
pub struct RecipeWithPoster {
    pub poster: Poster,
    pub id: i32,
    pub recipe_file_path: String,
    pub date_created: chrono::DateTime<Utc>,
}

impl Recipe {
    pub async fn get_paginated(
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

    pub async fn get_paginated_recipes_with_poster(
        pool: &Pool<Postgres>,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<RecipeWithPoster>, anyhow::Error> {
        let rows = sqlx::query(
            r#"SELECT u.uid, u.username, r.id, r.recipe_file_path, r.date_created FROM users u
            RIGHT OUTER JOIN recipes r
            ON u.uid = r.user_id 
            ORDER BY r.id LIMIT $1 OFFSET $2;"#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

        // We use .get as this should never fail, every recipe must have one of these values as
        // they are set to not null in the db
        let recipes: Vec<RecipeWithPoster> = rows
            .iter()
            .map(|row| RecipeWithPoster {
                poster: Poster {
                    uid: row.get("uid"),
                    username: row.get("username"),
                },
                id: row.get("id"),
                recipe_file_path: row.get("recipe_file_path"),
                date_created: row.get("date_created"),
            })
            .collect();

        Ok(recipes)
    }

    pub async fn get_by_id(pool: &Pool<Postgres>, recipe_id: i32) -> Result<Recipe, anyhow::Error> {
        let recipe = sqlx::query_as::<_, Recipe>(r#"SELECT * FROM recipes WHERE id = $1"#)
            .bind(recipe_id)
            .fetch_one(pool)
            .await?;

        Ok(recipe)
    }

    pub async fn insert(
        pool: &Pool<Postgres>,
        file_path: String,
        user_id: i32,
    ) -> Result<(), anyhow::Error> {
        let _ = sqlx::query(
            r#"INSERT INTO recipes (recipe_file_path, user_id)
        VALUES ( $1, $2 )"#,
        )
        .bind(file_path)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
