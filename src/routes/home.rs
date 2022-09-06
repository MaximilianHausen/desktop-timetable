use dioxus::prelude::*;

pub fn Page(cx: Scope) -> Element {
    rsx!(cx,
        p { "Total tolle Homepage" }
        Link {
            to: "/app",
            "Zum Dashboard"
        }
    )
}
