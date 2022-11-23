// external crates
use api_shared::prelude::LibError;
use axum::{Json};
// local modules
use crate::services::UserModel;

pub async fn post_users_route(Json(body): Json<UserModel>) -> Result<&'static str, LibError> {
    body.create_user_service()?;

    Ok("Success: account created")
}
