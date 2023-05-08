use std::net::SocketAddr;

use axum::extract::State;
use axum::extract::{ConnectInfo, Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{headers, TypedHeader};
use http::StatusCode;

use crate::models::prelude::*;
use crate::services::prelude::GameService;
use crate::AppState;

#[axum::debug_handler]
pub async fn join_room(
    Path(ticket): Path<Ticket>,
    State(app): State<AppState>,
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(_): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let ticket_data = if let Some(ticket_data) = app.ticket_service.get_ticket_data(&ticket).await {
        ticket_data
    } else {
        return (
            StatusCode::BAD_REQUEST,
            format!("Ticket {} not found", ticket),
        )
            .into_response();
    };
    let room_code = ticket_data.room_code();
    let room_id = app.lobby_service.get_room_id(room_code).await;
    let room_id = if let Some(room_id) = room_id {
        room_id
    } else {
        return (
            StatusCode::BAD_REQUEST,
            format!("Such room is not created"),
        )
            .into_response();
    };

    let sender = app.connection_event_sender.clone();

    ws.on_upgrade(move |socket| GameService::handle_socket(sender, socket, ticket_data, room_id))
}
