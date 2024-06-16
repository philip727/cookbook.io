use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use serde::{Deserialize, Serialize};

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
