use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{database::models::recipe::Poster, recipe_io::RecipeFileJson};

#[derive(Deserialize, Clone, Copy)]
pub struct GetRecipeQueryParams {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, MultipartForm)]
pub struct CreateRecipeForm {
    #[multipart(limit = "2MB")]
    pub thumbnail: Option<TempFile>,
    pub recipe: Text<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FullRecipePayload {
    pub recipe: RecipeFileJson,
    pub poster: Poster,
    pub id: i32,
    pub date_created: chrono::DateTime<Utc>,
    pub thumbnail: Option<String>
}
