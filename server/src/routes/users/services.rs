use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use anyhow::Context;
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    database::models::User,
    routes::{error::PrettyErrorResponse, users::helpers::is_password_valid},
};

#[get("/all")]
pub async fn get_all_users(db: Data<Pool<Postgres>>) -> impl Responder {
    let users = User::get_all_public(&db).await;

    if let Err(e) = users {
        let error = PrettyErrorResponse::new("Unable to get all users".into(), e.to_string());
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
        let error = PrettyErrorResponse::new(
            "No user found".into(),
            format!("Couldn't find user with the id: {}", id).into(),
        );

        return HttpResponse::NotFound().json(error);
    };

    HttpResponse::Ok().json(user)
}

#[derive(Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[post("/register")]
pub async fn register_user(payload: web::Json<RegisterPayload>, db: Data<Pool<Postgres>>) -> impl Responder {
    // Ensures its a valid username
    let username_re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !username_re.is_match(&payload.username) {
        let error = PrettyErrorResponse::new(
            "Invalid username".into(),
            format!(
                "The username: '{}' is invalid, it may only contain alphanumerical values and underscores", 
                payload.username
            ),
        );

        return HttpResponse::BadRequest().json(error);
    }

    if User::username_taken(&db, &payload.username).await {
        let error = PrettyErrorResponse::new(
            "Username already taken".into(),
            format!(
                "The username: '{}' is already taken", 
                payload.username
            ),
        );

        return HttpResponse::Conflict().json(error);
    }

    // Verifies the email
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_re.is_match(&payload.email) {
        let error = PrettyErrorResponse::new(
            "Invalid email".into(),
            format!(
                "The email: '{}' is invalid, please use a valid email format",
                payload.email
            ),
        );

        return HttpResponse::BadRequest().json(error);
    }

    if User::email_taken(&db, &payload.email).await {
        let error = PrettyErrorResponse::new(
            "Email already taken".into(),
            format!(
                "The email: '{}' is already taken", 
                payload.email
            ),
        );

        return HttpResponse::Conflict().json(error);
    }

    // Checks if password is secure enough
    if !is_password_valid(&payload.password) {
        let error = PrettyErrorResponse::new(
            "Invalid password".into(),
            "Your password is not strong enough. It must be at least 8 characters long, contain a combination of uppercase and lowercase letters, at least one digit and at least one special character".to_string()
        );

        return HttpResponse::BadRequest().json(error);
    }

    // Make sure passwords match on confirm
    if payload.password != payload.confirm_password {
        let error = PrettyErrorResponse::new(
            "Passwords do not match".into(),
            "Make sure your passwords match".to_string(),
        );

        return HttpResponse::BadRequest().json(error);
    }

    // Hash password
    let hash = hash(&payload.password, DEFAULT_COST);
    if let Err(e) = hash {
        let error = PrettyErrorResponse::new(
            "Failed to hash password".into(),
            e.to_string()
        );

        return HttpResponse::InternalServerError().json(error);
    }

    HttpResponse::Ok().body("Ok")
}
