use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use crate::{
    database::models::{recipe::Recipe, recipe_thumbnails::RecipeThumbnail}, extractors::auth::Authorized, pretty_error, recipe_io::RecipeFileJson, routes::{
        error::PrettyErrorResponse,
        recipes::helpers::{get_recipe_file, FullRecipePayload},
    }, static_files::helpers::rename_temp_file
};
use actix_multipart::form::MultipartForm;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{
    constants::RECIPE_DIR,
    helpers::{create_recipe_file, CreateRecipeForm, EditRecipeForm, GetRecipeQueryParams},
};

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

    let recipes = Recipe::get_paginated_recipes_with_poster(
        &pool,
        pagination.offset.unwrap(),
        pagination.limit.unwrap(),
    )
    .await;

    if let Err(e) = recipes {
        pretty_error!("Failed to get recipes".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    };
    let recipes = recipes.unwrap();

    let mut json_values: Vec<serde_json::Value> = Vec::new();
    for recipe in recipes.iter() {
        // Read recipe json
        let recipe_json = get_recipe_file(recipe);
        if let Err(e) = recipe_json {
            pretty_error!("Recipe file is invalid".to_string(), e.to_string(), error);

            return HttpResponse::InternalServerError().json(error);
        }
        let recipe_json = recipe_json.unwrap();

        // Push value to vec
        let value = json!({
            "poster": recipe.poster,
            "id": recipe.id,
            "title": recipe_json.title,
            "description": recipe_json.description,
            "thumbnail": recipe.thumbnail,
        });

        json_values.push(value);
    }

    HttpResponse::Ok().json(json_values)
}

#[get("by/{user_id}")]
pub async fn get_recipe_by_poster(
    pool: Data<Pool<Postgres>>,
    path: actix_web::web::Path<i32>,
) -> impl Responder {
    let uid = path.into_inner();

    let recipes = Recipe::get_by_poster(&pool, uid).await;
    if let Err(e) = recipes {
        pretty_error!(
            format!("Failed to get recipes by user with id: {}", uid),
            e.to_string(),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    let recipes = recipes.unwrap();
    let mut json_values: Vec<serde_json::Value> = Vec::new();

    for recipe in recipes.iter() {
        let recipe_json = get_recipe_file(recipe);
        if let Err(e) = recipe_json {
            pretty_error!("Recipe file is invalid".to_string(), e.to_string(), error);

            return HttpResponse::InternalServerError().json(error);
        }

        let recipe_json = recipe_json.unwrap();
        let value = json!({
            "poster": recipe.poster,
            "id": recipe.id,
            "title": recipe_json.title,
            "description": recipe_json.description,
            "thumbnail": recipe.thumbnail,
        });

        json_values.push(value)
    }

    if json_values.is_empty() {
        pretty_error!(
            "No recipes found".to_string(),
            format!("Couldn't find any recipes from the user with id: {}", uid),
            error
        );

        return HttpResponse::NotFound().json(error);
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

    let recipe_json = get_recipe_file(&recipe);
    if let Err(e) = recipe_json {
        pretty_error!("Recipe file is invalid".to_string(), e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    let recipe_json = recipe_json.unwrap();
    let full_recipe = FullRecipePayload {
        date_created: recipe.date_created,
        id: recipe.id,
        recipe: recipe_json,
        poster: recipe.poster,
        thumbnail: recipe.thumbnail,
    };

    HttpResponse::Ok().json(full_recipe)
}

pub async fn edit_recipe(
    authorized: Authorized,
    MultipartForm(form): MultipartForm<EditRecipeForm>,
    pool: Data<Pool<Postgres>>
) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    let recipe = Recipe::get_by_id(&pool, *form.recipe_id).await;
    if let Err(e) = recipe {
        pretty_error!(
            format!("Failed to get recipe with id: {}", *form.recipe_id),
            e.to_string(),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    let recipe = recipe.unwrap();
    if !(recipe.poster.uid == uid) {
        pretty_error!(
            format!("Failed to edit recipe"),
            "Poster id and submitter id do not match",
            error
        );

        return HttpResponse::Unauthorized().json(error);
    }

    let recipe_json = serde_json::from_str::<RecipeFileJson>(&form.recipe.to_string());
    if let Err(_err) = recipe_json {
        pretty_error!(
            "Invalid recipe",
            "The recipe is invalid, please ensure that all fields are filled out",
            error
        );

        return HttpResponse::BadRequest().json(error);
    }
    let recipe_json = recipe_json.unwrap();

    // Very unlikely, but better to be robust than not
    if !Path::new(RECIPE_DIR).exists() {
        let create_dir = fs::create_dir_all(RECIPE_DIR);

        if let Err(e) = create_dir {
            pretty_error!("Failed to create dir on recipe save", e.to_string(), error);

            return HttpResponse::InternalServerError().json(error);
        }
    }

    let file_path = RECIPE_DIR.to_owned() + &recipe.recipe_file_path;
    if let Err(e) = create_recipe_file(file_path.clone(), &recipe_json) {
        pretty_error!("Failed to save recipe file", e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    HttpResponse::Ok().body("Succesfully edited recipe")
}

// #[post(/create)]
pub async fn create_recipe(
    authorized: Authorized,
    MultipartForm(form): MultipartForm<CreateRecipeForm>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    let uuid = Uuid::new_v4();
    let file_name = uid.to_string() + "-" + &uuid.to_string();
    let file_path = RECIPE_DIR.to_owned() + &file_name;

    let recipe = serde_json::from_str::<RecipeFileJson>(&form.recipe.to_string());

    if let Err(_err) = recipe {
        pretty_error!(
            "Invalid recipe",
            "The recipe is invalid, please ensure that all fields are filled out",
            error
        );

        return HttpResponse::BadRequest().json(error);
    }
    let recipe = recipe.unwrap();

    if let Err(e) = recipe.is_valid_recipe() {
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

    if let Err(e) = create_recipe_file(file_path.clone(), &recipe) {
        pretty_error!("Failed to save recipe file", e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    let insert_recipe = Recipe::insert(&pool, file_name.clone(), uid).await;
    if let Err(e) = insert_recipe {
        pretty_error!(
            "Failed to insert recipe data into database",
            e.to_string(),
            error
        );

        return HttpResponse::InternalServerError().json(error);
    }
    let recipe_id = insert_recipe.unwrap();

    if let Some(temp_thumbnail_file) = form.thumbnail {
        let Some(mime_type) = &temp_thumbnail_file.content_type else {
            pretty_error!("Invalid thumbnail", "Couldn't get mime type", error);

            return HttpResponse::BadRequest().json(error);
        };

        if mime_type != &"image/jpeg" && mime_type != &"image/png" {
            pretty_error!(
                "Invalid thumbnail",
                "An invalid mime type was passed, only jpeg/png",
                error
            );

            return HttpResponse::BadRequest().json(error);
        }

        // Does not need to resolve or return
        // If it fails we just use default thumbnail
        match rename_temp_file(temp_thumbnail_file, "./thumbnails", &file_name) {
            Ok(file_name) => {
                let _ = RecipeThumbnail::insert_or_update(&pool, recipe_id, file_name).await;
            }
            _ => {}
        }
    };

    HttpResponse::Ok().body(recipe_id.to_string())
}

pub async fn can_edit(
    authorized: Authorized,
    path: actix_web::web::Path<i32>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    let recipe_id = path.into_inner();
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    let poster_id = Recipe::get_poster(&pool, recipe_id).await;
    if let Err(e) = poster_id {
        pretty_error!(
            format!("Failed to get poster of recipe with id: {}", recipe_id),
            e.to_string(),
            error
        );

        return HttpResponse::InternalServerError().json(error);
    }
    let poster_id = poster_id.unwrap();

    let authorized = poster_id == uid;
    let json = if authorized {
        let recipe = Recipe::get_by_id(&pool, recipe_id).await;
        if let Err(e) = recipe {
            pretty_error!(
                format!("Failed to get recipe with id: {}", recipe_id),
                e.to_string(),
                error
            );

            return HttpResponse::InternalServerError().json(error);
        }
        let recipe = recipe.unwrap();

        let recipe_json = get_recipe_file(&recipe);
        if let Err(e) = recipe_json {
            pretty_error!("Recipe file is invalid".to_string(), e.to_string(), error);

            return HttpResponse::InternalServerError().json(error);
        }

        let recipe_json = recipe_json.unwrap();
        let full_recipe = FullRecipePayload {
            date_created: recipe.date_created,
            id: recipe.id,
            recipe: recipe_json,
            poster: recipe.poster,
            thumbnail: recipe.thumbnail,
        };

        json!({
            "authorized": authorized,
            "recipe": full_recipe
        })
    } else {
        json!({
            "authorized": authorized
        })
    };

    HttpResponse::Ok().json(json)
}
