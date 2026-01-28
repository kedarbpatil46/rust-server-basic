use actix_web::web;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .service(handlers::home_handlers::greet),
    );
}