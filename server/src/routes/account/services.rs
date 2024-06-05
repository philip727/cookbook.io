use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::database::models::user::User;
use crate::routes::error::PrettyErrorResponse;
use crate::{middleware::auth::AuthExtension, pretty_error};

pub async fn verify_jwt(req: HttpRequest) -> impl Responder {
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


    let json = json!({
        "uid": uid,
        "username": auth.username
    });

    HttpResponse::Ok().json(json)
}
