use dioxus::prelude::*;

pub fn AuthPage(cx: Scope) -> Element {
    let code = match use_route(&cx).query_param("code") {
        Some(code) => code.to_string(),
        None => return rsx!(cx, Redirect { to: "/" }),
    };

    let success: &UseFuture<bool> = use_future(&cx, (), |()| async move {
        reqwest::Client::new()
            .post(crate::BACKEND_BASE_URL.to_string() + "homeworker/login")
            .body(code)
            .send()
            .await
            .is_ok()
    });

    match success.value() {
        Some(s) => match s {
            true => rsx!(cx, Redirect { to: "/app" }),
            false => rsx!(cx,
                p { "Bei der Authentifikation ist ein Fehler aufgetreten" }
                Link { to: "/", "Zurück" }
            )
        }
        None => None
    }
}
