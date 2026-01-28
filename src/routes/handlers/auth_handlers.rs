use actix_web::{Responder, post, web};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{self, SaltString}
};
use password_hash::rand_core::OsRng;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response, app_state, jwt::{encode_jwt}};

#[derive(Deserialize, Serialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(register_json.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(password_hash),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.id))
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let user = match entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&login_json.email))
        .one(&app_state.db)
        .await
        {
        Ok(Some(user)) => user,
        _ => {
            return api_response::ApiResponse::new(
                404,
                "User not found".to_string(),
            );
        }
    };

   let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return api_response::ApiResponse::new(
                500,
                "Invalid password hash".to_string(),
            );
        }    
   };

   if Argon2::default()
        .verify_password(login_json.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return api_response::ApiResponse::new(401, "Invalid password".to_string());
    }

    let token = encode_jwt(user.email, user.id.to_string());
    api_response::ApiResponse::new(200, format!("{{ 'token' : '{:?}'}}", token))
}
