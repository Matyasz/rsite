#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;
pub mod errors;
pub mod user_actions;
pub mod pages;

use actix_web::{web, App, HttpServer, middleware::Logger};
use tera::Tera;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;

use user_actions::login::{login, process_login};
use user_actions::logout::logout;
use user_actions::signup::{signup, process_signup};
use user_actions::submit_post::{submit, process_submission};
use user_actions::comment::comment;

use pages::post::post_page;
use pages::index::index;
use pages::user_profile::user_profile;

// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url =std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder().build(manager)
        .expect("Could not create postgres connection pool");
    
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&[0;32])
                    .name("auth-cookie")
                    .secure(false)
            ))
            .data(tera)
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/logout", web::to(logout))
            .route("/submit", web::get().to(submit))
            .route("/submit", web::post().to(process_submission))
            .service(
                web::resource("/post/{post_id}")
                    .route(web::get().to(post_page))
                    .route(web::post().to(comment))
            )
            .service(
                web::resource("/user/{username}")
                    .route(web::get().to(user_profile))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
