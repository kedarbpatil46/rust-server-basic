use actix_web::{App, HttpServer, middleware::Logger, web};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection };
use utils::app_state::AppState;

mod utils;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");   
        }
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let address = (*utils::constants::ADDRESS).clone();
    let port = (*utils::constants::PORT).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = match Database::connect(database_url).await {
        Ok(conn) => conn,
        Err(err) => panic!("Failed to connect to database: {}", err)
    };

    Migrator::up(&db, None).await.expect("Database migration failed");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { db: db.clone()}))
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
        .configure(routes::auth_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}