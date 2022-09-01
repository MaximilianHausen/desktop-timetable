use dioxus::prelude::*;

pub mod components;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    rsx!(cx,
        Router {
            //Route { to: "/", Home {} }
            //Route { to: "/auth", Auth {} }
            Route { to: "/", Dashboard {} }
            //Route { to: "", NotFound {} }
        }
    )
}

fn Dashboard(cx: Scope) -> Element {
    rsx!(cx,
        div {
            components::timetable::Timetable {}
        }
    )
}
