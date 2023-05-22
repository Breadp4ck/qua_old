use axum::{response::{IntoResponse, Response}, Json};
use http::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::json;



#[derive(Serialize, Deserialize)]
pub enum RoomError {
    RoomNotCreated,
    MissingPackage,
}

impl IntoResponse for RoomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RoomError::RoomNotCreated => (StatusCode::INTERNAL_SERVER_ERROR, "Can't create room"),
            RoomError::MissingPackage => (StatusCode::INTERNAL_SERVER_ERROR, "Missing package in request"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
