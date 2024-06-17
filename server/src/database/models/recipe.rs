use anyhow::Context;
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Poster {
    pub uid: i32,
    pub username: String,
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize)]
// A recipe struct without user id
pub struct RecipeWithPoster {
    pub poster: Poster,
    pub id: i32,
    pub recipe_file_path: String,
    pub date_created: chrono::DateTime<Utc>,
    pub thumbnail_path: Option<String>,
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
            r#"
            SELECT u.uid, u.username, pp.picture_path, r.id, r.recipe_file_path, r.date_created, rt.thumbnail_path FROM users u
            RIGHT OUTER JOIN recipes r
                ON u.uid = r.user_id
            LEFT OUTER JOIN user_details ud
                ON u.uid = ud.user_id
            LEFT OUTER JOIN profile_pictures pp
                ON u.uid = pp.user_id
            LEFT OUTER JOIN recipe_thumbnails rt
                ON rt.recipe_id = r.id
                    ORDER BY r.id LIMIT 10 OFFSET 0;"#,
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
                    picture: row.try_get("picture_path").unwrap_or(None),
                },
                id: row.get("id"),
                recipe_file_path: row.get("recipe_file_path"),
                date_created: row.get("date_created"),
                thumbnail_path: row.try_get("thumbnail_path").unwrap_or(None),
            })
            .collect();

        Ok(recipes)
    }

    pub async fn get_by_id(pool: &Pool<Postgres>, recipe_id: i32) -> Result<RecipeWithPoster, anyhow::Error> {
        let row = sqlx::query(r#"
        SELECT u.uid, u.username, pp.picture_path, r.id, r.recipe_file_path, r.date_created, rt.thumbnail_path FROM users u
            RIGHT OUTER JOIN recipes r
                ON u.uid = r.user_id
            LEFT OUTER JOIN user_details ud
                ON u.uid = ud.user_id
            LEFT OUTER JOIN profile_pictures pp
                ON u.uid = pp.user_id
            LEFT OUTER JOIN recipe_thumbnails rt
                ON rt.recipe_id = r.id
                    WHERE r.id = $1;"#)
            .bind(recipe_id)
            .fetch_one(pool)
            .await?;

        let recipes =
            RecipeWithPoster {
                poster: Poster {
                    uid: row.get("uid"),
                    username: row.get("username"),
                    picture: row.try_get("picture_path").unwrap_or(None),
                },
                id: row.get("id"),
                recipe_file_path: row.get("recipe_file_path"),
                date_created: row.get("date_created"),
                thumbnail_path: row.try_get("thumbnail_path").unwrap_or(None),
            };

        Ok(recipes)
    }

    pub async fn insert(
        pool: &Pool<Postgres>,
        file_path: String,
        user_id: i32,
    ) -> Result<i32, anyhow::Error> {
        let rec = sqlx::query(
            r#"INSERT INTO recipes (recipe_file_path, user_id)
        VALUES ( $1, $2 ) RETURNING id"#,
        )
        .bind(file_path)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        // Returns failed insert with message
        let recipe_id: i32 = rec.try_get("id").context("Failed to get recipe id")?;

        Ok(recipe_id)
    }
}
