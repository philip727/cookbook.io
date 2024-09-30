use actix_multipart::form::MultipartForm;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::database::models::profile_picture::ProfilePicture;
use crate::database::models::user::User;
use crate::database::models::user_details::UserDetails;
use crate::extractors::auth::Authorized;
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
    MultipartForm(form): MultipartForm<UploadPictureForm>,
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

    let payload = serde_json::from_str::<UpdateUserDetailsPayload>(&form.details.to_string());
    if let Err(err) = payload {
        pretty_error!("Invalid details", err.to_string(), error);

        return HttpResponse::BadRequest().json(error);
    }

    let payload = payload.unwrap();
    if let Err(e) = payload.verify() {
        return e;
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

    if let Some(picture) = form.picture {
        match rename_temp_file(picture, "./profile_pictures", &uid.to_string()) {
            Ok(file_name) => {
                if let Err(e) =
                    ProfilePicture::insert_or_update(&pool, uid, file_name.clone()).await
                {
                    pretty_error!("Invalid profile picture", e.to_string(), error);

                    return HttpResponse::InternalServerError().json(error);
                };

                HttpResponse::Ok().body("Account details succesfully updated")
            }
            Err(e) => {
                pretty_error!("Invalid upload", e.to_string(), error);

                HttpResponse::BadRequest().json(error)
            }
        }
    } else {
        HttpResponse::Ok().body("Account details succesfully updated")
    }
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
