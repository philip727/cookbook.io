use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow},
    Pool, Postgres,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub amount: i16,
    pub recipe_id: i32,
}

impl Ingredient {
    pub async fn insert(pool: &Pool<Postgres>, measurement: String, amount: i16, recipe_id: i32) -> Result<(), anyhow::Error> {
        let _ = sqlx::query(
            r#"INSERT INTO ingredients (name, amount, recipe_id) 
            VALUES ( $1, $2, $3 )"#,
        )
        .bind(measurement)
        .bind(amount)
        .bind(recipe_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_from_recipe(
        pool: &Pool<Postgres>,
        recipe_id: i32,
    ) -> Result<Vec<Ingredient>, anyhow::Error> {
        let rows =
            sqlx::query_as::<_, Ingredient>(r#"SELECT * FROM ingredients WHERE recipe_id = $1"#)
                .bind(recipe_id)
                .fetch_all(pool)
                .await?;

        Ok(rows)
    }

    pub async fn delete(pool: &Pool<Postgres>, recipe_id: i32) -> Result<(), anyhow::Error> {
        let _ = sqlx::query(r#"DELETE FROM ingredients WHERE recipe_id = $1"#)
            .bind(recipe_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
