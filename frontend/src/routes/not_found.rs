use dioxus::prelude::*;

pub fn Page(cx: Scope) -> Element {
    rsx!(cx,
        p { "Hier sehen sie eine tolle 404-Seite" }
        Link {
            to: "/",
            "Zurück zur Hauptseite"
        }
    )
}
