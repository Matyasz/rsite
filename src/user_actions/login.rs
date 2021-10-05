#[path = "../models.rs"]
pub mod models;
use models::{User, LoginUser};

#[path = "../schema.rs"]
pub mod schema;

#[path = "../errors/mod.rs"]
pub mod errors;

use actix_web::{web, HttpResponse, Responder};
use actix_identity::Identity;
use tera::{Tera, Context};
use argonautica::Verifier;
use dotenv::dotenv;

use errors::server_error::ServerError;

use r2d2;
use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2::ConnectionManager};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn login(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    if let Some(id) = id.identity() {
        return HttpResponse::Ok().body(format!("Already logged in as {}", id));
    }

    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn process_login(data: web::Form<LoginUser>, pool: web::Data<Pool>, id: Identity) -> Result<HttpResponse, ServerError> {
    use schema::users::dsl::{username, users};

    let connection = pool.get()?;
    let user = users.filter(username.eq(&data.username)).first::<User>(&connection)?;

    dotenv().ok();
    let secret = std::env::var("SECRET_KEY")?;

    let valid = Verifier::default()
        .with_hash(user.password)
        .with_password(data.password.clone())
        .with_secret_key(secret)
        .verify()?;
    
    if valid {
        let session_token = String::from(user.username);
        id.remember(session_token);

        Ok(HttpResponse::Ok().body(format!("Logged in as {}", data.username)))
    }
    else {
        Ok(HttpResponse::Ok().body("Incorrect password"))
    }
}