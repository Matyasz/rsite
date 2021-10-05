#[path = "../models.rs"]
pub mod models;
use models::{User, Post, Comment};

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

pub async fn user_profile(tera: web::Data<Tera>,
                      pool: web::Data<Pool>,
                      web::Path(user_id): web::Path<String>) -> Result<HttpResponse, ServerError> {
    use schema::users::dsl::{username, users};

    let connection = pool.get()?;
    let user: User = users.filter(username.eq(&user_id))
        .get_result(&connection)?;

    let posts: Vec<Post> = Post::belonging_to(&user)
        .load(&connection)?;
    
    let comments: Vec<Comment> = Comment::belonging_to(&user)
        .load(&connection)?;
    
    let mut data = Context::new();
    data.insert("title", &format!("{}'s Profile", user.username));
    data.insert("user", &user);
    data.insert("posts", &posts);
    data.insert("comments", &comments);
    
    let rendered = tera.render("profile.html", &data).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}