use crate::{database::models::user::User, routes::{error::PrettyErrorResponse, recipes::helpers::GetRecipeQueryParams}};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use crate::{database::models::recipes::Recipe, pretty_error};

#[get("/all")]
pub async fn get_all_recipes(
    pool: Data<Pool<Postgres>>,
    mut pagination: web::Query<GetRecipeQueryParams>,
) -> impl Responder {
    // Ensure we are getting no more than 10 as this would be costly
    pagination.limit = match pagination.limit {
        Some(limit) => {
            if limit > 10 {
                Some(10)
            } else {
                Some(limit)
            }
        }
        None => Some(10),
    };

    if let None = pagination.offset {
        pagination.offset = Some(0);
    }

    let recipes =
        Recipe::get_with_pagination(&pool, pagination.offset.unwrap(), pagination.limit.unwrap())
            .await;


    if let Err(e) = recipes {
        pretty_error!("Failed to get recipes".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    }

    let recipes = recipes.unwrap();
    // Adds the user information to the request as well
    let mut json_values: Vec<Value> = Vec::new();
    for recipe in recipes.iter() {
        let user = User::get_by_id(&pool, recipe.user_id).await;
        let Ok(user) = user else {
            continue;
        };

        let Some(user) = user else {
            continue;
        };

        let json = json!({
            "id": recipe.id,
            "title": recipe.title,
            "description": recipe.description,
            "date_created": recipe.date_created,
            "poster": {
                "uid": user.uid,
                "username": user.username
            }
        });

        json_values.push(json);
    }

    HttpResponse::Ok().json(json_values)
}
