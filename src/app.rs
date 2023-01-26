use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::home::*;
use crate::routes::login::*;

#[cfg(feature = "ssr")]
#[derive(Clone, Copy)]
pub struct HomeworkerContext {
    pub client_id: ReadSignal<String>,
    pub access_token: ReadSignal<Option<String>>,
    pub refresh_token: ReadSignal<Option<String>>,
}

#[cfg(feature = "ssr")]
pub fn get_cookie(
    headers: &http::HeaderMap<http::HeaderValue>,
    cookie_name: &str,
) -> Option<String> {
    Some(
        headers
            .get("Cookie")?
            .to_str()
            .unwrap()
            .split("; ")
            .filter(|s| s.starts_with(cookie_name))
            .next()?
            .split_once('=')
            .unwrap()
            .1
            .to_owned(),
    )
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let headers = use_context::<leptos_axum::RequestParts>(cx).map(|r| r.headers).unwrap_or_default();
            let (client_id, _) = create_signal(cx, std::env::var("HW_CLIENT_ID").unwrap());
            let (access_token, _) = create_signal(cx, get_cookie(&headers, "access-token"));
            let (refresh_token, _) = create_signal(cx, get_cookie(&headers, "refresh-token"));
            provide_context(
                cx,
                HomeworkerContext {
                    client_id: client_id,
                    access_token: access_token,
                    refresh_token: refresh_token,
                },
            );
        }
    }

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/desktop_timetable.css"/>
        <Style>
            "@font-face {
                font-family: 'Rubik';
                font-style: normal;
                font-weight: 400;
                src: url('./fonts/rubik-v23-latin-regular.woff2') format('woff2');
            }"
        </Style>

        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! { cx, <HomePage/> } />
                <Route path="/login" view=move |cx| view! { cx, <LoginPage/> } />
            </Routes>
        </Router>
    }
}
