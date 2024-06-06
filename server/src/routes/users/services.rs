use std::collections::BTreeMap;

use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    auth::helpers::get_signed_jwt_token,
    database::models::user::User,
    helpers::is_alnum_whitespace,
    pretty_error,
    routes::{
        error::PrettyErrorResponse,
        users::helpers::{LoginIdentifier, LoginPayload, RegisterPayload},
    },
};

#[get("/all")]
pub async fn get_all_users(db: Data<Pool<Postgres>>) -> impl Responder {
    let users = User::get_all_public(&db).await;

    if let Err(e) = users {
        pretty_error!("Unable to get all users".to_string(), e.to_string(), error);
        return HttpResponse::InternalServerError().json(error);
    }

    HttpResponse::Ok().json(users.unwrap())
}

#[get("/{id}")]
pub async fn get_user_by_id(db: Data<Pool<Postgres>>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let user = User::get_by_id_public(&db, id).await;

    if let Err(e) = user {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let Some(user) = user.unwrap() else {
        pretty_error!(
            "No user found".to_string(),
            format!("Couldn't find user with the id: {}", id),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    HttpResponse::Ok().json(user)
}

#[post("/register")]
pub async fn register_user(
    payload: web::Json<RegisterPayload>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    // Ensures its a valid username
    if !User::username_is_valid(&payload.username) {
        pretty_error!(
            "Invalid username".to_string(),
            format!(
                "The username: '{}' is invalid, it may only contain alphanumerical values and underscores", 
                payload.username
            ),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    if User::has_username_been_used(&pool, &payload.username).await {
        pretty_error!(
            "Username already taken".to_string(),
            format!("The username: '{}' is already taken", payload.username),
            error
        );

        return HttpResponse::Conflict().json(error);
    }

    // Verifies the email
    if !User::email_is_valid(&payload.email) {
        pretty_error!(
            "Invalid email".to_string(),
            format!(
                "The email: '{}' is invalid, please use a valid email format",
                payload.email
            ),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    if User::has_email_been_used(&pool, &payload.email).await {
        pretty_error!(
            "Email already taken".to_string(),
            format!("The email: '{}' is already taken", payload.email),
            error
        );

        return HttpResponse::Conflict().json(error);
    }

    // Checks if password is secure enough
    if !User::is_password_valid(&payload.password) {
        pretty_error!(
            "Invalid password".to_string(),
            "Your password must contain at least 8 characters, a combination of uppercase and lowercase characters, at least one digit and at least one special character".to_string(),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    // Makes sure the two passwords provided are the same
    if payload.password != payload.confirm_password {
        pretty_error!(
            "Passwords do not match".to_string(),
            "Your passwords do match, please enter a new password".to_string(),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    let hash = hash(&payload.password, DEFAULT_COST);
    if let Err(e) = hash {
        pretty_error!("Failed to hash password".to_string(), e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    let insert_user = User::insert(&pool, &payload.username, &payload.email, &hash.unwrap()).await;
    if let Err(e) = insert_user {
        pretty_error!("Register user failed".to_string(), e.to_string(), error);

        return HttpResponse::InternalServerError().json(error);
    }

    let (uid, username) = insert_user.unwrap();
    HttpResponse::Ok().json(json!({"uid": uid, "username": username}))
}

#[post("/login")]
pub async fn login_user(
    payload: web::Json<LoginPayload>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    let identifier: Option<LoginIdentifier>;
    if User::email_is_valid(&payload.identifier) {
        identifier = Some(LoginIdentifier::Email(payload.identifier.to_string()));
    } else {
        if !is_alnum_whitespace(&payload.identifier) {
            pretty_error!(
                "The identifier is invalid".to_string(),
                format!("Please provide a username(only alphanumerical) or email"),
                error
            );

            return HttpResponse::BadRequest().json(error);
        }

        identifier = Some(LoginIdentifier::Username(payload.identifier.to_string()));
    }

    // If no identifier was provided then we must return
    if identifier.is_none() {
        pretty_error!(
            "No identifier was provided".to_string(),
            format!("Please provide a username or email"),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    let user = match identifier.unwrap() {
        // We need unique errors so we check for none inside of match instead
        LoginIdentifier::Username(username) => {
            let user = User::get_by_name(&pool, &username).await.unwrap_or(None);
            if let None = user {
                pretty_error!(
                    "No user".to_string(),
                    format!("There is no user with the username: {}", username),
                    error
                );

                return HttpResponse::BadRequest().json(error);
            }

            user.unwrap()
        }
        LoginIdentifier::Email(email) => {
            let user = User::get_by_email(&pool, &email).await.unwrap_or(None);
            if let None = user {
                pretty_error!(
                    "No user".to_string(),
                    format!("There is no user with the email: {}", email),
                    error
                );

                return HttpResponse::BadRequest().json(error);
            }

            user.unwrap()
        }
    };

    let Ok(pw_match) = bcrypt::verify(&payload.password, &user.password) else {
        pretty_error!(
            "Failed to verify password".to_string(),
            "Failed to verify the password provided, please contact support".to_string(),
            error
        );

        return HttpResponse::BadRequest().json(error);
    };

    if !pw_match {
        pretty_error!(
            "Incorrect password".to_string(),
            "The password provided was incorrect, please try again".to_string(),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    // The claims for out jwt
    let mut claims = BTreeMap::new();
    claims.insert("uid".to_string(), user.uid.to_string());
    claims.insert("username".to_string(), user.username);
    let jwt_token = get_signed_jwt_token(claims);

    let jwt_payload = json!({
        "jwt": jwt_token.as_str().to_string(),
    });

    HttpResponse::Ok().json(jwt_payload)
}

pub fn get_profile_picture() {

}
