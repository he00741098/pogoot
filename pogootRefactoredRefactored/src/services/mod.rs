mod database;
mod notecard;
mod pogoot;
mod search_engine;
pub mod server;
mod user_manage;

mod CFStorage;
use serde::Serialize;
use serde_json::to_string;
// pub fn to_response_shortcut<g>(item:g)->impl IntoResponse
// where g:Serialize
// {
//     let item = to_string(&item);
//     if item.is_err(){
//         // return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
//     }
//     item.unwrap().into_response()
// }
//
// pub fn deal_with_result(result:Result<A,B>)->
