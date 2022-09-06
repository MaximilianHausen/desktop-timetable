pub mod components;
pub mod routes;

use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    rsx!(cx,
        Router {
            Route { to: "/", routes::home::Page {} }
            //Route { to: "/auth", Auth {} }
            Route { to: "/app", routes::dashboard::Page {} }
            Route { to: "", routes::not_found::Page {} }
        }
    )
}
