// external crates
use api_shared::prelude::LibError;
use axum::{Json};
// local modules
use crate::services::UserForm;

pub async fn post_users_route(Json(body): Json<UserForm>) -> Result<&'static str, LibError> {
    body.create_user_service()?;

    Ok("Success: account created")
}
