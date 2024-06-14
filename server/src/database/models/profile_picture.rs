use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(FromRow, Serialize, Deserialize)]
pub struct ProfilePicture {
    pub user_id: i32,
    pub picture_path: Option<String>,
}

impl ProfilePicture {
    pub async fn insert_or_update(
        pool: &Pool<Postgres>,
        user_id: i32,
        picture_path: String,
    ) -> Result<(), anyhow::Error> {
        let rec = sqlx::query(
            r#"INSERT INTO profile_pictures(picture_path, user_id) 
            VALUES ($1, $2) ON CONFLICT (user_id) 
            DO UPDATE SET picture_path = excluded.picture_path, user_id = excluded.user_id"#,
        )
        .bind(picture_path)
        .bind(user_id)
        .execute(pool)
        .await;

        if let Err(e) = rec {
            let err = e.to_string();
            return Err(anyhow!("Failed to insert user: {}", err));
        }

        Ok(())
    }

    pub async fn get_by_user_id(
        pool: &Pool<Postgres>,
        user_id: i32,
    ) -> Result<Option<ProfilePicture>, anyhow::Error> {
        let row = sqlx::query_as::<_, ProfilePicture>(
            r#"SELECT * FROM profile_pictures WHERE user_id = $1"#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await;

        if let Err(e) = row {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(anyhow!(
                "Failed to find profile_picture of user with id: {}",
                user_id
            ));
        }

        Ok(Some(row.unwrap()))
    }
}
