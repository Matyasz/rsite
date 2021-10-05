#[path = "../models.rs"]
pub mod models;
use models::{User, Post, Comment, NewComment, CommentForm};

#[path = "../schema.rs"]
pub mod schema;

#[path = "../errors/mod.rs"]
pub mod errors;

use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use r2d2;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};

use errors::server_error::ServerError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn comment(data: web::Form<CommentForm>,
                 pool: web::Data<Pool>,
                 id: Identity,
                 web::Path(post_id): web::Path<i32>) -> Result<HttpResponse, ServerError> {
    if let Some(id) = id.identity() {
        use schema::posts::dsl::posts;
        use schema::users::dsl::{users, username};

        let connection = pool.get()?;

        let post: Post = posts.find(post_id)
            .get_result(&connection)?;
        
        let user: Result<User, diesel::result::Error> = users
            .filter(username.eq(id))
            .first(&connection);
        
        match user {
            Ok(u) => {
                let parent_id = None;
                let new_comment = NewComment::new(
                    data.comment.clone(), post.id, u.id, parent_id);
                
                use schema::comments;
                diesel::insert_into(comments::table)
                    .values(&new_comment)
                    .get_result::<Comment>(&connection)?;
                
                return Ok(HttpResponse::Ok().body("Comment successfully posted"));
            }
            Err(e) => {
                println!("{:?}", e);
                return Ok(HttpResponse::Ok().body("Comment successfully posted"));
            }
        }
    }

    Ok(HttpResponse::Unauthorized().body("Please log in to comment"))
}