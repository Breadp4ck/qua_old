use dioxus::prelude::*;
use dioxus_router::*;

pub mod components;
pub mod contexts;
pub mod pages;
pub mod services;

use components::prelude::*;
use pages::prelude::*;

// const JOIN_ROOM_API: &str = "ws://localhost:8000/api/room/join/12345";

fn main() {
    // init debug tool for WebAssembly
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
    cx.render(rsx! (
        Router {
            self::nav {}
            Route { to: "/", self::home {}}
            Route { to: "/create", self::create {}}
            Route { to: "/join", self::join {}}
            Route { to: "/game", self::game {}}
            Route { to: "", self::not_found {}}
        }
    ))
}
