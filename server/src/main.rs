use std::time::Duration;

use actix_cors::Cors;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
//use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{
    middleware::Logger,
    web::{self, scope, to, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use middleware::auth::Authentication;
use routes::{
    account::services::*,
    recipes::services::{
        can_edit, create_recipe, edit_recipe, get_recipe, get_recipe_by_poster, get_recipes,
    },
    users::services::{get_all_users, get_user_by_id, login_user, register_user},
};
use sqlx::postgres::PgPoolOptions;

pub mod auth;
pub mod database;
pub mod extractors;
pub mod helpers;
pub mod middleware;
pub mod recipe_io;
pub mod routes;
pub mod static_files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(10))
        .max_connections(50)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Couldnt conect to postgres db");

    let backend = InMemoryBackend::builder().build();

    //    let store = MemoryStore::new();
    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 200)
            .real_ip_key()
            .build();

        let limiter_middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();

        App::new()
            .wrap(cors)
            .wrap(limiter_middleware)
            //           .wrap(
            //               RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
            //                   .with_interval(Duration::from_secs(60))
            //                   .with_max_requests(100),
            //           )
            .app_data(Data::new(pool.clone()))
            .service(
                scope("/v1")
                    .service(
                        actix_files::Files::new("/thumbnails", "./thumbnails").show_files_listing(),
                    )
                    .service(
                        actix_files::Files::new("/pfp", "./profile_pictures").show_files_listing(),
                    )
                    .service(
                        scope("/users")
                            .service(get_all_users)
                            .service(get_user_by_id)
                            .service(register_user)
                            .service(login_user),
                    )
                    .service(
                        scope("/account")
                            .wrap(Authentication)
                            .service(web::resource("/verify").route(web::get().to(verify_jwt)))
                            .service(web::resource("/").route(web::get().to(get_account_details)))
                            .service(
                                web::resource("/update_details")
                                    .route(web::post().to(update_account_details)),
                            )
                            .service(
                                web::resource("/delete_pfp")
                                    .route(web::get().to(delete_profile_picture)),
                            ),
                    )
                    .service(
                        scope("/recipes")
                            .service(
                                web::resource("/create")
                                    .wrap(Authentication)
                                    .route(web::post().to(create_recipe)),
                            )
                            .service(
                                web::resource("/request_edit/{recipe_id}")
                                    .wrap(Authentication)
                                    .route(web::get().to(can_edit)),
                            )
                            .service(
                                web::resource("/edit")
                                    .wrap(Authentication)
                                    .route(web::post().to(edit_recipe)),
                            )
                            .service(get_recipe_by_poster)
                            .service(get_recipes)
                            .service(get_recipe),
                    )
                    .wrap(Logger::default()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
