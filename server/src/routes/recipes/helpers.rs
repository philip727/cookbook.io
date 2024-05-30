use serde::{Deserialize, Serialize};

use crate::database::models::{recipes::Recipe, user::User};

#[derive(Deserialize, Clone, Copy)]
pub struct GetRecipeQueryParams {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FullRecipeDetails {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub date_created: chrono::DateTime<chrono::Utc>,
    pub poster: RecipePoster,
}

impl FullRecipeDetails {
    pub fn new(recipe: Recipe, user: User) -> Self {
        Self {
            id: recipe.id,
            title: recipe.title,
            description: recipe.description,
            date_created: recipe.date_created,
            poster: RecipePoster {
                uid: user.uid,
                username: user.username,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipePoster {
    pub uid: i32,
    pub username: String,
}
