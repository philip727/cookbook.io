use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use crate::{
    database::models::recipe::Recipe, middleware::auth::AuthExtension, pretty_error, recipe_io::RecipeFileJson, routes::error::PrettyErrorResponse
};
use actix_web::{
    web::{self, Data},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::constants::RECIPE_DIR;

// #[post(/create)]
pub async fn create_recipe(
    req: HttpRequest,
    payload: web::Json<RecipeFileJson>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    let extensions = req.extensions();
    let auth = extensions.get::<AuthExtension>();

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
        pretty_error!("Failed to insert recipe data into database", e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    HttpResponse::Ok().body("Succesfully created recipe")
}

pub async fn get_ingredients() {}
