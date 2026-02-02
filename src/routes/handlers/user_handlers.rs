use actix_web::{get, patch, web::{self, Json}};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response, app_state, jwt::Claims};

#[get("")]
pub async fn get_user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let user = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(404, "User Not Found".to_string()));

    let user_data = user.unwrap();
    Ok(api_response::ApiResponse::new(
        200,
        format!("{{ 'name': '{}', 'email': {}}}", user_data.name, user_data.email)
    ))
}

#[derive(Serialize, Deserialize, Clone)]
struct UpdateDataModel {
    name: String,
    email: String
}
#[patch("/update")]
pub async fn update_user(
    app_state: web::Data<app_state::AppState>,
    update_data: web::Json<UpdateDataModel>,
    claim_data: Claims
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let mut update_user = entity::user::Entity::find_by_id(claim_data.id)
    .one(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .ok_or(api_response::ApiResponse::new(400, "Could not fetch User to update".to_string()))?
    .into_active_model();

    update_user.name = Set(update_data.name.clone());
    update_user.email = Set(update_data.email.clone());
    update_user.update(&app_state.db).await.map_err(|err| api_response::ApiResponse::new(500, err.to_string()));
    
    Ok(api_response::ApiResponse::new(200, format!("User updated successfully {{ 'name': {}, 'email': {} }}", update_data.name, update_data.email)))
}
