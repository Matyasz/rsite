#[path = "../models.rs"]
pub mod models;
use models::{User};

#[path = "../schema.rs"]
pub mod schema;

#[path = "../errors/mod.rs"]
pub mod errors;

use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};
use actix_identity::Identity;
use r2d2;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};

use models::{Post, NewPost, PostForm};
use errors::server_error::ServerError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn submit(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a post");

    if let Some(_id) = id.identity() {
        let rendered = tera.render("submission.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered);
    }

    HttpResponse::Unauthorized().body("Please log in to submit a post")
}

pub async fn process_submission(data: web::Form<PostForm>, pool: web::Data<Pool>, id: Identity) -> Result<HttpResponse, ServerError> {
    if let Some(id) = id.identity() {
        use schema::users::dsl::{username, users};
        
        let connection = pool.get()?;
        let user: Result<User, diesel::result::Error> = users.filter(username.eq(id)).first(&connection);

        match user {
            Ok(u) => {
                use schema::posts;
                
                let new_post = NewPost::from_post_form(data.into_inner(), u.id);

                diesel::insert_into(posts::table)
                    .values(&new_post)
                    .get_results::<Post>(&connection)?;
                
                return Ok(HttpResponse::Ok().body("Successfully posted"));
                
            }
            Err(e) => {
                println!("{:?}", e);

                return Ok(HttpResponse::Ok().body("User not found"));
            }
        }
    }

    Ok(HttpResponse::Unauthorized().body("Please log in to submit a post"))
}