use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::home::*;
use crate::routes::login::*;

#[derive(Clone, Copy)]
pub struct HomeworkerContext {
    pub client_id: ReadSignal<String>,
    pub access_token: ReadSignal<Option<String>>,
    pub refresh_token: ReadSignal<Option<String>>,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

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
