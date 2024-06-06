use std::{
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::Path,
};

use crate::{
    database::models::{recipe::Recipe, user::User},
    middleware::auth::AuthenticationExtension,
    pretty_error,
    recipe_io::RecipeFileJson,
    routes::error::PrettyErrorResponse,
};
use actix_web::{
    get,
    web::{self, Data, Json},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{constants::RECIPE_DIR, helpers::GetRecipeQueryParams};

#[get("/all")]
pub async fn get_recipes(
    pool: Data<Pool<Postgres>>,
    mut pagination: web::Query<GetRecipeQueryParams>,
) -> impl Responder {
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
        Recipe::get_paginated(&pool, pagination.offset.unwrap(), pagination.limit.unwrap()).await;

    if let Err(e) = recipes {
        pretty_error!("Failed to get recipes".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    };
    let recipes = recipes.unwrap();

    let mut json_values: Vec<serde_json::Value> = Vec::new();
    for recipe in recipes.iter() {
        let user = User::get_by_id(&pool, recipe.user_id).await;

        let Ok(user) = user else {
            continue;
        };

        let Some(user) = user else {
            continue;
        };

        // Read recipe json
        let file_path = RECIPE_DIR.to_string() + recipe.recipe_file_path.as_str();
        let file = File::open(file_path);
        if let Err(e) = file {
            pretty_error!(
                "Recipe file doesn't exist".to_string(),
                e.to_string(),
                error
            );

            return HttpResponse::InternalServerError().json(error);
        }
        let mut file = file.unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let recipe_json = serde_json::from_str::<RecipeFileJson>(&data);

        if let Err(e) = recipe_json {
            pretty_error!("Recipe file is invalid".to_string(), e.to_string(), error);

            return HttpResponse::InternalServerError().json(error);
        }
        let recipe_json = recipe_json.unwrap();

        // Push value to vec
        let value = json!({
            "poster": {
                "uid": user.uid,
                "username": user.username
            },
            "recipe": {
                "id": recipe.id,
                "title": recipe_json.title,
                "description": recipe_json.description
            }
        });

        json_values.push(value);
    }

    HttpResponse::Ok().json(json_values)
}

#[get("/{id}")]
pub async fn get_recipe(
    pool: Data<Pool<Postgres>>,
    path: actix_web::web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    let recipe = Recipe::get_by_id(&pool, id).await;
    if let Err(e) = recipe {
        pretty_error!(
            format!("Failed to get recipe with id: {}", id),
            e.to_string(),
            error
        );

        return HttpResponse::NotFound().json(error);
    };
    let recipe = recipe.unwrap();

    let file_path = RECIPE_DIR.to_string() + recipe.recipe_file_path.as_str();
    let file = File::open(file_path);
    if let Err(e) = file {
        pretty_error!(
            "Recipe file doesn't exist".to_string(),
            e.to_string(),
            error
        );

        return HttpResponse::InternalServerError().json(error);
    }

    let mut file = file.unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let recipe_json = serde_json::from_str::<RecipeFileJson>(&data);

    if let Err(e) = recipe_json {
        pretty_error!("Recipe file is invalid".to_string(), e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }
    let recipe_json = recipe_json.unwrap();

    HttpResponse::Ok().json(recipe_json)
}

// #[post(/create)]
pub async fn create_recipe(
    req: HttpRequest,
    payload: web::Json<RecipeFileJson>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    let extensions = req.extensions();
    let auth = extensions.get::<AuthenticationExtension>();

    // Need to make sure we can actually get the auth details from extension
    let Some(auth) = auth else {
        pretty_error!(
            "Unauthorized".to_string(),
            "Unable to get auth details from extension",
            error
        );
        return HttpResponse::Unauthorized().json(error);
    };

    let Ok(uid) = auth.uid.parse::<i32>() else {
        pretty_error!(
            "Unauthorized".to_string(),
            "Invalid uid passed in auth",
            error
        );

        return HttpResponse::InternalServerError().json(error);
    };

    if let Err(e) = payload.is_valid_recipe() {
        pretty_error!("Invalid recipe format", e.to_string(), error);

        return HttpResponse::BadRequest().json(error);
    };

    if !Path::new(RECIPE_DIR).exists() {
        let create_dir = fs::create_dir_all(RECIPE_DIR);

        if let Err(e) = create_dir {
            pretty_error!("Failed to create dir on recipe save", e.to_string(), error);

            return HttpResponse::InternalServerError().json(error);
        }
    }

    let uuid = Uuid::new_v4();
    let file_name = uid.to_string() + "-" + &uuid.to_string();
    let file_path = RECIPE_DIR.to_owned() + &file_name;
    let file = File::create(file_path.clone());

    if let Err(e) = file {
        pretty_error!("Failed to save recipe file", e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    let file = file.unwrap();
    let mut writer = BufWriter::new(file);
    let write = serde_json::to_writer(&mut writer, &payload);

    if let Err(e) = write {
        pretty_error!("Failed to write to recipe file", e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    let flush = writer.flush();

    if let Err(e) = flush {
        pretty_error!("Failed to flush recipe writer", e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    if let Err(e) = Recipe::insert(&pool, file_name, uid).await {
        pretty_error!(
            "Failed to insert recipe data into database",
            e.to_string(),
            error
        );

        return HttpResponse::InternalServerError().json(error);
    }

    HttpResponse::Ok().body("Succesfully created recipe")
}
