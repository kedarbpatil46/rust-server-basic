use actix_web::{middleware::from_fn, web::{self, service}};

use crate::routes::middlewares;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(handlers::user_handlers::get_user)
    );
}