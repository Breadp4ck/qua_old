use dioxus::prelude::*;
use dioxus_router::*;
use ewebsock::{WsReceiver, WsSender};

pub mod components;
pub mod contexts;
pub mod pages;
pub mod services;

use components::prelude::*;
use pages::prelude::*;
use qua_game::game::Game;

struct GameInstance(Option<Game>);
struct GameWsSender(Option<WsSender>);
struct GameWsReceiver(Option<WsReceiver>);

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
    use_shared_state_provider(cx, || GameInstance(None));
    use_shared_state_provider(cx, || GameWsSender(None));
    use_shared_state_provider(cx, || GameWsReceiver(None));

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
