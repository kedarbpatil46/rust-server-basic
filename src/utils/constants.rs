
use lazy_static::lazy_static;
use std::env;

lazy_static!{
    pub static ref ADDRESS:String = set_address();
    pub static ref PORT:u16 = set_port();
    pub static ref DATABASE_URL:String = set_database();
    pub static ref JWT_SECRET:String = set_jwt_secret();
}

fn set_address() -> String {
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

fn set_database() -> String {
    env::var("DATABASE_URL").unwrap()
}

fn set_jwt_secret() -> String {
    env::var("JWT_SECRET_KEY").unwrap()
}