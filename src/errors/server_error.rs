use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ServerError {
    ArgonauticaError,
    DieselError,
    EnvError,
    R2D2Error,
    UserError(String)
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test")
    }
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::ArgonauticaError => HttpResponse::InternalServerError().json("Argonautica Error"),
            ServerError::DieselError => HttpResponse::InternalServerError().json("Diesel Error"),
            ServerError::EnvError => HttpResponse::InternalServerError().json("Environment Error"),
            ServerError::R2D2Error => HttpResponse::InternalServerError().json("R2D2 Error"),
            ServerError::UserError(data) => HttpResponse::InternalServerError().json(data)
        }
    }
}

impl From<std::env::VarError> for ServerError {
    fn from(_: std::env::VarError) -> ServerError {
        log::error!("Var error");
        ServerError::EnvError
    }
}

impl From<r2d2::Error> for ServerError {
    fn from(_: r2d2::Error) -> ServerError {
        log::error!("r2d2 error");
        ServerError::R2D2Error
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            diesel::result::Error::NotFound => {
                log::error!("{:?}", err);
                ServerError::UserError("User not found".to_string())
            },
            _ => ServerError::DieselError
        }
    }
}

impl From<argonautica::Error> for ServerError {
    fn from(_: argonautica::Error) -> ServerError {
        log::error!("Argonautica error");
        ServerError::ArgonauticaError
    }
}