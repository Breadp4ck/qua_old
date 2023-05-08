mod game_service;
mod lobby_service;
mod ticket_service;

pub mod prelude {
    pub use super::*;

    pub use game_service::*;
    pub use lobby_service::*;
    pub use ticket_service::*;
}
