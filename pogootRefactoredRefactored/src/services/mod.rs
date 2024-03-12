mod database;
mod notecard;
mod pogoot;
mod user_manage;
pub mod corporate;

use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::to_string;
/// Casts an item to string
pub fn to_response_shortcut<g>(item:g)->impl IntoResponse
where g:Serialize
{
    let item = to_string(&item);
    if item.is_err(){
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
    item.unwrap().into_response()
}
