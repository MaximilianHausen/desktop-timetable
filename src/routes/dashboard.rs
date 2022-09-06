use dioxus::prelude::*;

pub fn Page(cx: Scope) -> Element {
    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",

            height: "calc(100vh - 16px)",

            div {}

            crate::components::timetable::Timetable {}

            div {}
        }
    )
}
