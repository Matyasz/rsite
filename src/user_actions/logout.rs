use actix_web::{HttpResponse, Responder};
use actix_identity::Identity;

pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out")
}