use anyhow::anyhow;
use sqlx::{Pool, Postgres};

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
}
