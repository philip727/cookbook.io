use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, types::Json, Pool, Postgres, Row};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub uid: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    // Creates new user
    pub async fn insert(
        pool: &Pool<Postgres>,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(i32, String), anyhow::Error> {
        let rec = sqlx::query(
            r#"
            INSERT INTO users (username, email, password) 
            VALUES ( $1, $2, $3 ) RETURNING uid, username"#,
        )
        .bind(username)
        .bind(email)
        .bind(password)
        .fetch_one(pool)
        .await;

        // Returns failed insert with message
        if let Err(e) = rec {
            let err = e.to_string();
            return Err(e).context(format!("Failed to insert user: {}", err));
        }

        // Returns the uid and username back to requester
        let rec = rec.unwrap();
        let uid: i32 = rec.try_get("uid").context("Failed to get uid")?;
        let username: String = rec.try_get("username").context("Failed to get username")?;

        Ok((uid, username))
    }

    // Gets all users with publicly safe data
    pub async fn get_all_public(pool: &Pool<Postgres>) -> Result<Vec<Value>, anyhow::Error> {
        let rows = sqlx::query(r#"SELECT uid, username FROM users ORDER by uid"#)
            .fetch_all(pool)
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
        pool: &Pool<Postgres>,
        id: i32,
    ) -> Result<Option<Value>, anyhow::Error> {
        let row = sqlx::query(r#"SELECT uid, username FROM users WHERE uid = $1"#)
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
        pool: &Pool<Postgres>,
        name: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = $1"#)
            .bind(name)
            .fetch_one(pool)
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
        pool: &Pool<Postgres>,
        email: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE email = $1"#)
            .bind(email)
            .fetch_one(pool)
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
    pub async fn username_taken(pool: &Pool<Postgres>, name: &str) -> bool {
        let user = Self::get_by_name(&pool, name).await.unwrap_or(None);

        user.is_some()
    }

    // Queries for a user with that email and checks if we get a result
    pub async fn email_taken(pool: &Pool<Postgres>, email: &str) -> bool {
        let user = Self::get_by_email(&pool, email).await.unwrap_or(None);

        user.is_some()
    }
}
