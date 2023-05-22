use axum::{
    extract::{Multipart, State},
    Json,
};

use crate::{errors::prelude::*, models::prelude::*, AppState};

#[axum::debug_handler]
pub async fn create_room(
    State(app): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<RoomCode>, RoomError> {
    let (_, data) = if let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        (name, data)
    } else {
        return Err(RoomError::MissingPackage);
    };

    let (room_id, room_code) = app.lobby_service.create_room(&data).await;
    app.game_service.create_room(room_id).await;

    Ok(Json(room_code))
}
