use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_router::*;
use fermi::prelude::*;

pub mod components;
pub mod contexts;
pub mod pages;
pub mod services;

use components::prelude::*;
use pages::prelude::*;
use services::prelude::{Ticket, RoomCode};
use tokio::sync::Mutex;
use wasm_sockets::PollingClient;

static TICKET: Atom<Option<Ticket>> = |_| None;
static ROOM_CODE: Atom<Option<RoomCode>> = |_| None;

type InnerConnection = Arc<Mutex<PollingClient>>;
type Connection = Option<InnerConnection>;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn not_found(cx: Scope) -> Element {
    cx.render(rsx! {
        Redirect { to: "/" }
    })
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);

    cx.render(rsx! (
        Router {
            self::nav {}
            Route { to: "/", self::home {}}
            Route { to: "/create", self::create {}}
            Route { to: "/join", self::join {}}
            Route { to: "/room", self::room {}}
            Route { to: "", self::not_found {}}
        }
    ))
}
