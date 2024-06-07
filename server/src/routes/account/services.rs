use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::database::models::user::User;
use crate::routes::error::PrettyErrorResponse;
use crate::{middleware::auth::AuthenticationExtension, pretty_error};

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

pub async fn get_account_details(
    pool: Data<Pool<Postgres>>,
    req: HttpRequest
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

    let user = User::get_by_id(&pool, uid).await;
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

    let json = json!({
        "uid": uid,
        "username": user.username,
        "email": user.email,
    });

    HttpResponse::Ok().json(json)
}
