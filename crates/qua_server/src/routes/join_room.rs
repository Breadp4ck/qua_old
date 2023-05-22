use std::net::SocketAddr;

use axum::extract::State;
use axum::extract::{ConnectInfo, Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{headers, TypedHeader};
use http::StatusCode;

use crate::models::prelude::*;
use crate::services::prelude::*;
use crate::AppState;

#[axum::debug_handler]
pub async fn join_room(
    Path(ticket): Path<String>,
    State(app): State<AppState>,
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(_): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let ticket = Ticket::from(ticket);
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

    if app.room_service.exists(room_code.clone()).await {
        let sender = app.room_event_sender.clone();
        ws.on_upgrade(move |socket| {
            RoomService::handle_socket(sender, socket, ticket_data, room_code)
        })
    } else {
        return (StatusCode::BAD_REQUEST, format!("Such room is not created")).into_response();
    }
}
