use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    database::models::recipe::{Poster, RecipeWithPoster},
    recipe_io::RecipeFileJson,
};

use super::constants::RECIPE_DIR;

#[derive(Deserialize, Clone, Copy)]
pub struct GetRecipeQueryParams {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, MultipartForm)]
pub struct CreateRecipeForm {
    #[multipart(limit = "2MB")]
    pub thumbnail: Option<TempFile>,
    pub recipe: Text<String>,
}

#[derive(Debug, MultipartForm)]
pub struct EditRecipeForm {
    pub thumbnail: Option<TempFile>,
    pub recipe: Text<String>,
    pub recipe_id: Text<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FullRecipePayload {
    pub recipe: RecipeFileJson,
    pub poster: Poster,
    pub id: i32,
    pub date_created: chrono::DateTime<Utc>,
    pub thumbnail: Option<String>,
}

pub fn create_recipe_file(file_path: String, recipe: &RecipeFileJson) -> anyhow::Result<()> {
    let file = File::create(file_path.clone())?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, recipe)?;
    writer.flush()?;

    Ok(())
}

pub fn get_recipe_file(recipe: &RecipeWithPoster) -> anyhow::Result<RecipeFileJson> {
    let file_path = RECIPE_DIR.to_string() + recipe.recipe_file_path.as_str();
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let recipe_json = serde_json::from_str::<RecipeFileJson>(&data)?;

    Ok(recipe_json)
}
