use std::path::{Path, PathBuf};
use std::str::FromStr;

use actix_multipart::form::MultipartForm;
use actix_web::web::{self, Data};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::database::models::profile_picture::ProfilePicture;
use crate::database::models::user::User;
use crate::database::models::user_details::UserDetails;
use crate::extractors::auth::Authorized;
use crate::helpers::is_alnum_whitespace_and_ex_chars;
use crate::pretty_error;
use crate::routes::error::PrettyErrorResponse;
use crate::static_files::helpers::rename_temp_file;

use super::helpers::{UpdateUserDetailsPayload, UploadPictureForm};

pub async fn verify_jwt(authorized: Authorized) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    let json = json!({
        "uid": uid,
        "username": username
    });

    HttpResponse::Ok().json(json)
}

pub async fn get_account_details(
    pool: Data<Pool<Postgres>>,
    authorized: Authorized,
) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    let user = User::get_details(&pool, uid).await;
    if let Err(e) = user {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let Some(user) = user.unwrap() else {
        pretty_error!(
            "No user found".to_string(),
            format!("Couldn't find user with the id: {}", uid),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    HttpResponse::Ok().json(user)
}

pub async fn update_account_details(
    payload: web::Json<UpdateUserDetailsPayload>,
    pool: Data<Pool<Postgres>>,
    authorized: Authorized,
) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    if let Some(pronouns) = &payload.pronouns {
        if !UserDetails::is_pronoun(pronouns) {
            pretty_error!(
                "This pronoun is invalid".to_string(),
                "Please follow the rule of 'pronoun1/pronoun2'",
                error
            );
            return HttpResponse::BadRequest().json(error);
        }
    }

    if let Some(bio) = &payload.bio {
        if !is_alnum_whitespace_and_ex_chars(bio) {
            pretty_error!(
                "This bio is invalid".to_string(),
                "Please only use alphanumerical characters",
                error
            );
            return HttpResponse::BadRequest().json(error);
        }
    }

    if let Some(location) = &payload.location {
        if !is_alnum_whitespace_and_ex_chars(location) {
            pretty_error!(
                "This location name is invalid".to_string(),
                "Please only use alphanumerical characters",
                error
            );
            return HttpResponse::BadRequest().json(error);
        }
    }

    if let Err(e) = UserDetails::insert_or_create(
        &pool,
        &payload.bio,
        &payload.pronouns,
        &payload.location,
        uid,
    )
    .await
    {
        pretty_error!(
            "Failed to update user details".to_string(),
            e.to_string(),
            error
        );
        return HttpResponse::InternalServerError().json(error);
    };

    HttpResponse::Ok().body("Account details succesfully updated")
}

#[actix_web::get("/pfp/{user_id}")]
pub async fn get_profile_picture(
    pool: Data<Pool<Postgres>>,
    path: actix_web::web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let pfp = ProfilePicture::get_by_user_id(&pool, id).await;
    if let Err(e) = pfp {
        pretty_error!("No profile picture found".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    };

    let Some(pfp) = pfp.unwrap() else {
        pretty_error!(
            "No profile picture found".to_string(),
            format!("Couldn't find profile picture from the user id: {}", id),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    HttpResponse::Ok().json(pfp)
}

pub async fn delete_profile_picture(
    authorized: Authorized,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        // Should never pass
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    match ProfilePicture::delete_by_user_id(&pool, uid).await {
        Ok(..) => HttpResponse::Ok().body("Succesfully deleted profile picture"),
        Err(e) => {
            pretty_error!("Failed to delete profile picture", e.to_string(), error);

            HttpResponse::InternalServerError().json(error)
        }
    }
}

pub async fn upload_profile_picture(
    MultipartForm(form): MultipartForm<UploadPictureForm>,
    authorized: Authorized,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    if let Authorized::Failed(reason) = authorized {
        pretty_error!("Unauthorized", reason, error);

        return HttpResponse::Unauthorized().json(error);
    }

    let Authorized::Passed(uid, _username) = authorized else {
        panic!("Despite the if let authorized::failed, we still panicked");
    };

    let Some(mime_type) = &form.picture.content_type else {
        pretty_error!("Invalid upload", "Couldn't get mime type", error);

        return HttpResponse::BadRequest().json(error);
    };

    if mime_type != &"image/jpeg" && mime_type != &"image/png" {
        pretty_error!(
            "Invalid upload",
            "An invalid mime type was passed, only jpeg/png",
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    // Saves the temp file
    match rename_temp_file(form.picture, "./profile_pictures", &uid.to_string()) {
        Ok(file_name) => {
            if let Err(e) = ProfilePicture::insert_or_update(&pool, uid, file_name.clone()).await {
                pretty_error!("Invalid upload", e.to_string(), error);

                return HttpResponse::InternalServerError().json(error);
            };

            HttpResponse::Ok().body(file_name)
        }
        Err(e) => {
            pretty_error!("Invalid upload", e.to_string(), error);

            HttpResponse::BadRequest().json(error)

        }
    }
}
