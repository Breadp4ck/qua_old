use axum::{extract::State, Json};

use crate::AppState;
use crate::errors::prelude::*;
use crate::models::prelude::*;

#[axum::debug_handler]
pub async fn obtain_ticket(
    State(state): State<AppState>,
    Json(ticket_data): Json<TicketData>,
) -> Result<Json<Ticket>, RoomError> {
    let ticket = state.ticket_service.add_ticket(ticket_data).await;

    Ok(Json(ticket))
}
