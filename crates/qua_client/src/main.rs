use std::{sync::Arc, time::Duration};

use contexts::package_resource::PackageResource;
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::prelude::*;

pub mod components;
pub mod contexts;
pub mod pages;
pub mod services;

use components::prelude::*;
use pages::prelude::*;
use services::prelude::*;
use tokio::sync::Mutex;
use wasm_sockets::PollingClient;

#[derive(PartialEq, Eq, Clone, Copy)]
enum PersonType {
    Lead,
    Player,
    Host,
}

static INFO: Atom<String> = |_| "You have joined the game.".into();
static TICKET: Atom<Option<Ticket>> = |_| None;
static TIMER: Atom<Option<Duration>> = |_| None;
static PERSON_TYPE: Atom<PersonType> = |_| PersonType::Player;
static ROOM_CODE: Atom<Option<RoomCode>> = |_| None;
static PACKAGE_RESOURCE: Atom<Option<Arc<Mutex<PackageResource>>>> = |_| None;

type InnerConnection = Arc<Mutex<PollingClient>>;
type Connection = Option<InnerConnection>;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn not_found(cx: Scope) -> Element {
    cx.render(rsx! { Redirect { to: "/" } })
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let prevent_reload = js_sys::Function::new_no_args("return 'Are you sure?'");
    web_sys::window().unwrap().set_onbeforeunload(Some(&prevent_reload));

    use_shared_state_provider(cx, || ErrorMessage(None));

    cx.render(rsx! (
        Router { 
            error_popup {}
            self::nav {}
            Route { to: "/", self::home {} }
            Route { to: "/create", self::create {} }
            Route { to: "/join", self::join {} }
            Route { to: "/room", self::room {} }
            Route { to: "/package", self::package_editor {} }
            Route { to: "", self::not_found {} }
        }
    ))
}
