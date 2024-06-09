use actix_web::web::{self, Data};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::database::models::user::User;
use crate::database::models::user_details::UserDetails;
use crate::helpers::{is_alnum_whitespace, is_alnum_whitespace_and_ex_chars};
use crate::routes::error::PrettyErrorResponse;
use crate::{middleware::auth::AuthenticationExtension, pretty_error};

use super::helpers::UpdateUserDetailsPayload;

pub async fn verify_jwt(req: HttpRequest) -> impl Responder {
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

    let json = json!({
        "uid": uid,
        "username": auth.username
    });

    HttpResponse::Ok().json(json)
}

pub async fn get_account_details(pool: Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {
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
    req: HttpRequest,
    payload: web::Json<UpdateUserDetailsPayload>,
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

    if let Some(display_name) = &payload.display_name {
        if !is_alnum_whitespace(display_name) {
            pretty_error!(
                "This display name is invalid".to_string(),
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
        &payload.display_name,
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
