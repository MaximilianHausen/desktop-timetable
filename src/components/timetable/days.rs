use dioxus::prelude::*;

pub fn WeekHeader(cx: Scope) -> Element {
    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            gap: "var(--large-gap-size)",

            Day { day: "Monday" }
            Day { day: "Tuesday" }
            Day { day: "Wednesday" }
            Day { day: "Thursday" }
            Day { day: "Friday" }
        }
    )
}

#[inline_props]
pub fn Day<'a>(cx: Scope, day: &'a str) -> Element {
    rsx!(cx,
        div {
            width: "var(--lesson-size)",
            height: "calc(2.5 * var(--base-unit))",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "10px",

            "{day}"
        }
    )
}
