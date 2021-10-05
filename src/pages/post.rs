#[path = "../models.rs"]
pub mod models;
use models::{User, Comment, Post};

#[path = "../schema.rs"]
pub mod schema;

#[path = "../errors/mod.rs"]
pub mod errors;

use actix_web::{web, HttpResponse};
use tera::{Tera, Context};
use actix_identity::Identity;

use errors::server_error::ServerError;

use r2d2;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn post_page(
    tera: web::Data<Tera>,
    pool: web::Data<Pool>,
    id: Identity,
    web::Path(post_id): web::Path<i32>) -> Result<HttpResponse, ServerError> {
        use schema::posts::dsl::posts;
        use schema::users::dsl::users;

        let connection = pool.get()?;

        let post: Post = posts.find(post_id)
            .get_result(&connection)?;
        
        let user: User = users.find(post.author)
            .get_result(&connection)?;

        let comments: Vec<(Comment, User)> = Comment::belonging_to(&post)
            .inner_join(users)
            .load(&connection)?;

        let mut data = Context::new();
        data.insert("title", &format!("{} - rsite", post.title));
        data.insert("post", &post);
        data.insert("user", &user);
        data.insert("comments", &comments);

        if let Some(_id) = id.identity() {
            data.insert("logged_in", "true");
        } else {
            data.insert("logged_in", "false");
        }
        
        let rendered = tera.render("post.html", &data).unwrap();
        Ok(HttpResponse::Ok().body(rendered))
}