use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetRecipeQueryParams {
    pub offset: u32,
    pub limit: u32,
}
