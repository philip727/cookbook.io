use actix_web::{
    web::{self, scope, to, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use routes::{
    recipes::services::{create_recipe, get_recipes},
    users::services::{get_all_users, get_user_by_id, login_user, register_user},
};
use sqlx::postgres::PgPoolOptions;

pub mod auth;
pub mod database;
pub mod helpers;
pub mod middleware;
pub mod recipe_io;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Couldnt conect to postgres db");

    HttpServer::new(move || {
        App::new().app_data(Data::new(pool.clone())).service(
            scope("/v1")
                .service(
                    scope("/users")
                        .service(get_all_users)
                        .service(get_user_by_id)
                        .service(register_user)
                        .service(login_user),
                )
                .service(
                    scope("/recipes")
                        .service(
                            web::resource("/create")
                                .wrap(AuthMiddleware)
                                .route(web::post().to(create_recipe)),
                        )
                        .service(get_recipes),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
