use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, Pool, Postgres, Row};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub uid: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    // Gets all users with publicly safe data
    pub async fn get_all_public(db: &Pool<Postgres>) -> Result<Vec<Value>, anyhow::Error> {
        let rows = sqlx::query(r#"SELECT uid, username FROM users"#)
            .fetch_all(db)
            .await?;

        let mut values = Vec::new();
        for row in rows.iter() {
            let uid: i32 = row.try_get("uid").context("Failed to get uid")?;
            let username: String = row.try_get("username").context("Failed to get username")?;

            values.push(json!({
                "uid": uid,
                "username": username,
            }));
        }

        Ok(values)
    }

    // Get a user by their id with publicly safe data
    pub async fn get_by_id_public(
        db: &Pool<Postgres>,
        id: i32,
    ) -> Result<Option<Value>, anyhow::Error> {
        let row = sqlx::query(r#"SELECT uid, username FROM users WHERE uid = $1"#)
            .bind(id)
            .fetch_one(db)
            .await;

        if let Err(e) = row {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(e).context(format!("Failed to find user with id: {}", id));
        }

        let row = row.unwrap();
        let uid: i32 = row.try_get("uid").context("Failed to get uid")?;
        let username: String = row.try_get("username").context("Failed to get username")?;

        Ok(Some(json!({
            "uid": uid,
            "username": username,
        })))
    }

    // Get a user by their name
    pub async fn get_by_name(
        db: &Pool<Postgres>,
        name: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = $1"#)
            .bind(name)
            .fetch_one(db)
            .await;

        if let Err(e) = user {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(e).context(format!("Failed to find user with name: {}", name));
        }

        let user = user.unwrap();
        Ok(Some(user))
    }

    // Get a user by their email
    pub async fn get_by_email(
        db: &Pool<Postgres>,
        email: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE email = $1"#)
            .bind(email)
            .fetch_one(db)
            .await;

        if let Err(e) = user {
            // If we dont find one, the query was still a success but we have no result
            if let sqlx::Error::RowNotFound = e {
                return Ok(None);
            }

            return Err(e).context(format!("Failed to find user with email: {}", email));
        }

        let user = user.unwrap();
        Ok(Some(user))
    }

    // Queries for a user with that name and checks if we get a result
    pub async fn username_taken(db: &Pool<Postgres>, name: &str) -> bool {
        let user = Self::get_by_name(&db, name).await.unwrap_or(None);

        user.is_some()
    }

    // Queries for a user with that email and checks if we get a result
    pub async fn email_taken(db: &Pool<Postgres>, email: &str) -> bool {
        let user = Self::get_by_email(&db, email).await.unwrap_or(None);

        user.is_some()
    }
}
