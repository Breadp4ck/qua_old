use std::net::SocketAddr;

use axum::body::{Body, StreamBody};
use axum::extract::State;
use axum::extract::{ConnectInfo, Path, WebSocketUpgrade};
use axum::response::{AppendHeaders, IntoResponse, Response};
use axum::{headers, TypedHeader};
use http::{header, StatusCode};
use tokio_util::io::ReaderStream;

use crate::models::prelude::*;
use crate::services::prelude::GameService;
use crate::AppState;

pub async fn get_room_package(
    Path(room_code): Path<String>,
    State(app): State<AppState>,
) -> impl IntoResponse {
    let room_code = RoomCode::from(room_code);
    let package = app.lobby_service.get_package(room_code).await;

    let body = Body::from(package);

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "application/zip"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"pack.qua\"",
        ),
    ]);

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap();

    response
}
