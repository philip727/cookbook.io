use regex::Regex;
use sqlx::{Pool, Postgres};

pub struct UserDetails {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub location: Option<String>,
    pub user_id: i32,
}

impl UserDetails {
    pub async fn insert_or_create(
        pool: &Pool<Postgres>,
        bio: &Option<String>,
        pronouns: &Option<String>,
        location: &Option<String>,
        user_id: i32,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"INSERT INTO user_details (bio, pronouns, location, user_id)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (user_id) DO UPDATE
                    SET bio = excluded.bio, 
                        pronouns = excluded.pronouns, 
                        location = excluded.location;"#,
        )
        .bind(bio)
        .bind(pronouns)
        .bind(location)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn is_pronoun(s: &str) -> bool {
        let re = Regex::new(r"^\w+/+\w+$").unwrap();

        re.is_match(s)
    }
}
