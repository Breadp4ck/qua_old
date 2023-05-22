use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use http::header;
// use sqlx::postgres::PgPool;
// use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod errors;
mod models;
mod routes;
mod services;

use models::prelude::*;
use routes::prelude::*;
use services::prelude::*;
// use errors::prelude::*;

const UPLOADS_DIRECTORY: &str = "uploads";
const MAX_TICKET_ID_LENGTH: usize = 12;
const MAX_ROOM_CODE_LENGTH: usize = 5;
const TICKET_EXPIRE_TIME_SECONDS: u64 = 60;

#[derive(Clone)]
pub struct AppState {
    pub ticket_service: TicketService,
    pub room_service: RoomService,
    pub room_event_sender: UnboundedSender<RoomServiceEvent>,
    // pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_jwt=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let db_connection_str = std::env::var("127.0.0.1:5432")
    //     .unwrap_or_else(|_| "postgres://postgres:mysecretpassword@localhost".to_string());

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .acquire_timeout(Duration::from_secs(3))
    //     .connect(&db_connection_str)
    //     .await
    //     .expect("can't connect to database");

    let cors = CorsLayer::new()
        .allow_headers([header::ORIGIN, header::CONTENT_TYPE])
        .allow_origin(Any);

    let ticket_service = TicketService::new();
    let room_service = RoomService::new();
    let room_event_sender = room_service.sender();

    let state = AppState {
        ticket_service,
        room_service,
        room_event_sender,
        // pool,
    };

    match std::fs::create_dir(UPLOADS_DIRECTORY) {
        Err(why) => log::warn!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let app = Router::new()
        .route("/api/room/join/:ticket", get(join_room))
        .route("/api/room/obtain_ticket", post(obtain_ticket))
        .route("/api/room/create", post(create_room))
        .route("/api/room/package/:room_code", get(get_room_package))
        .layer(cors)
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
