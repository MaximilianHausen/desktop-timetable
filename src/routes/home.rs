use leptos::*;
use leptos_router::*;

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Total tolle homepage"</p>
        <A href="/app">"Zum Dashboard"</A>
    }
}
