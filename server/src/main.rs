use actix_cors::Cors;
use actix_web::{
    web::{self, scope, to, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use routes::{
    account::services::verify_jwt,
    recipes::services::{create_recipe, get_recipe, get_recipes},
    users::services::{get_all_users, get_user_by_id, login_user, register_user},
};
use sqlx::postgres::PgPoolOptions;

pub mod auth;
pub mod database;
pub mod helpers;
pub mod middleware;
pub mod recipe_io;
pub mod routes;
pub mod static_files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Couldnt conect to postgres db");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .service(
                scope("/v1")
                    .service(actix_files::Files::new("/thumbnails", "./thumbnails"))
                    .service(actix_files::Files::new("/pfp", "./profile_pictures"))
                    .service(
                        scope("/users")
                            .service(get_all_users)
                            .service(get_user_by_id)
                            .service(register_user)
                            .service(login_user),
                    )
                    .service(
                        scope("/account").service(
                            web::resource("/verify")
                                .wrap(AuthMiddleware)
                                .route(web::get().to(verify_jwt)),
                        ),
                    )
                    .service(
                        scope("/recipes")
                            .service(
                                web::resource("/create")
                                    .wrap(AuthMiddleware)
                                    .route(web::post().to(create_recipe)),
                            )
                            .service(get_recipes)
                            .service(get_recipe),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
