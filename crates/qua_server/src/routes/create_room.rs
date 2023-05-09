use axum::{extract::State, Json};

use crate::{errors::prelude::*, models::prelude::*, AppState};

#[axum::debug_handler]
pub async fn create_room(
    State(app): State<AppState>,
    // Json(_): Json<RoomCode>, // TODO: extract package
) -> Result<Json<RoomCode>, RoomError> {
    let (room_id, room_code) = app.lobby_service.create_room().await;
    app.game_service.create_room(room_id).await;

    Ok(Json(room_code))
}
