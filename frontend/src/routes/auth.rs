use dioxus::prelude::*;

//TODO: Fix auth page
pub fn AuthPage(cx: Scope) -> Element {
    let a = "";
    let code = match use_route(&cx).query_param("code") {
        Some(code) => code,
        None => return rsx!(cx, Redirect { to: "" }),
    };
    let state = match use_route(&cx).query_param("state") {
        Some(state) => {
            //TODO: Verify oauth state param
            state
        },
        None => return rsx!(cx, Redirect { to: "" }),
    };
    homeworker::auth::exchange_token("a".to_string(), "b".to_string(), code.to_string());
    rsx!(cx, "")
}
