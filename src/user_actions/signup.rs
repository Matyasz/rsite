#[path = "../models.rs"]
pub mod models;
use models::{User, NewUser};

#[path = "../schema.rs"]
pub mod schema;

#[path = "../errors/mod.rs"]
pub mod errors;

use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};

use errors::server_error::ServerError;

use r2d2;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn process_signup(data: web::Form<NewUser>, pool: web::Data<Pool>) -> Result<HttpResponse, ServerError> {
    use schema::users;

    let connection = pool.get()?;

    let new_user = NewUser::new(
        data.username.clone(),
        data.email.clone(),
        data.password.clone());

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_results::<User>(&connection)?;
    
    println!("{:?}", data);
    Ok(HttpResponse::Ok().body(format!("Successfully registered user: {}", data.username)))
}