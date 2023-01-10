use std::string::ToString;

use dioxus::prelude::*;
use homeworker::HomeworkerClient;

pub mod components;
pub mod routes;
pub mod types;

pub const HW_CLIENT_HEADER: &str = "desktop-dashboard";
pub const HW_CLIENT_ID: &str = std::env!("CLIENT_ID");
pub const BACKEND_BASE_URL: &str = std::env!("BACKEND_URL");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    rsx!(cx,
        style {
            "@font-face {{
                font-family: 'Rubik';
                font-style: normal;
                font-weight: 400;
                src: url('./fonts/rubik-v23-latin-regular.woff2') format('woff2');
            }}"
        }

        Router {
            Route { to: "/", routes::home::HomePage {} }
            Route { to: "/login", routes::login::LoginPage {} }
            Route { to: "/app", routes::dashboard::DashboardPage {} }
            Route { to: "/auth", routes::auth::AuthPage {} }
            Route { to: "", routes::not_found::NotFoundPage {} }
        }
    )
}

//TODO: Replace homeworker client management
pub fn new_hw_client() -> HomeworkerClient {
    HomeworkerClient::with_custom_url(
        "_".to_string(),
        HW_CLIENT_HEADER.to_string(),
        crate::BACKEND_BASE_URL.to_string() + "homeworker/api/v2",
    )
}

pub mod state {
    use std::time::Duration;

    use dioxus::fermi::Atom;
    use gloo_storage::Storage;

    pub static UPDATE_RATE: Atom<Duration> = |_| {
        Duration::from_secs(
            gloo_storage::LocalStorage::get("homeworker_refresh_rate").unwrap_or(3600),
        )
    };
}
