use axum::body::Body;
use axum::extract::State;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

use crate::models::prelude::*;
use crate::AppState;

pub async fn get_room_package(
    Path(room_code): Path<String>,
    State(app): State<AppState>,
) -> impl IntoResponse {
    let room_code = RoomCode::from(room_code);
    let package = app.room_service.get_package(room_code).await;

    let body = Body::from(package);

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap();

    response
}
