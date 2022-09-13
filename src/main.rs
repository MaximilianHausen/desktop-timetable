pub mod components;
pub mod routes;
pub mod types;

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

pub mod state {
    use std::time::Duration;
    use crate::types::timetable::{Timetable, TimetableColumn};
    use dioxus::fermi::Atom;
    use gloo_storage::Storage;

    pub static UPDATE_RATE: Atom<Duration> = |_| Duration::from_secs(gloo_storage::LocalStorage::get("homeworker_refresh_rate").unwrap_or(3600));

    pub static TIMETABLE: Atom<Timetable> = |_| Timetable {
        times: vec![],
        columns: vec![
            TimetableColumn {
                name: "Monday".to_owned(),
                lessons: vec![],
            },
            TimetableColumn {
                name: "Tuesday".to_owned(),
                lessons: vec![],
            },
            TimetableColumn {
                name: "Wednesday".to_owned(),
                lessons: vec![],
            },
            TimetableColumn {
                name: "Thursday".to_owned(),
                lessons: vec![],
            },
            TimetableColumn {
                name: "Friday".to_owned(),
                lessons: vec![],
            }
        ],
    };
}
