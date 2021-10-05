#[path = "../models.rs"]
pub mod models;
use models::{User, Post};

#[path = "../schema.rs"]
pub mod schema;

#[path = "../errors/mod.rs"]
pub mod errors;

use actix_web::{web, HttpResponse};
use tera::{Tera, Context};

use errors::server_error::ServerError;

use r2d2;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index(tera: web::Data<Tera>, pool: web::Data<Pool>) -> Result<HttpResponse, ServerError> {
    use schema::posts::dsl::posts;
    use schema::users::dsl::users;

    let connection = pool.get()?;

    let all_posts :Vec<(Post, User)> = posts.inner_join(users)
        .load(&connection)?;

    let mut data = Context::new();
    data.insert("title", "Taylor's Rust Site");
    data.insert("post_users", &all_posts);

    let rendered = tera.render("index.html", &data).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}