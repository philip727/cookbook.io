use crate::routes::{error::PrettyErrorResponse, recipes::helpers::GetRecipeQueryParams};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use crate::{database::models::recipes::Recipe, pretty_error};

#[get("/all")]
pub async fn get_all_recipes(
    pool: Data<Pool<Postgres>>,
    mut pagination: web::Query<GetRecipeQueryParams>,
) -> impl Responder {
    // Ensure we are getting no more than 10 as this would be costly
    pagination.limit = if pagination.limit > 10 {
        10
    } else {
        pagination.limit
    };

    let recipes = Recipe::get_with_pagination(&pool, pagination.offset, pagination.limit).await;
    if let Err(e) = recipes {
        pretty_error!("Failed to get recipes".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    }

    HttpResponse::Ok().json(recipes.unwrap())
}
