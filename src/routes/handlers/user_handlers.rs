use actix_web::{Responder, get, web};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{routes::handlers::user_handlers, utils::{api_response, app_state}};



#[get("")]
pub async fn get_user(
    app_state: web::Data<app_state::AppState>
) -> impl Responder{
    // let user = entity::user::Entity::find()
    // .filter(entity::user::Column::Id.eq());
    api_response::ApiResponse::new(200, "Verified User".to_string())
}