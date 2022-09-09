use dioxus::prelude::*;

#[inline_props]
pub fn WeekHeader(cx: Scope, days: Vec<String>) -> Element {
    let week_header_elements = days
        .iter()
        .map(|day| rsx! {
            Day {
                name: day,
            }
        });

    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            gap: "var(--large-gap-size)",

            week_header_elements
        }
    )
}

#[inline_props]
pub fn Day<'a>(cx: Scope, name: &'a str) -> Element {
    rsx!(cx,
        div {
            width: "var(--lesson-size)",
            height: "calc(2.5 * var(--base-unit))",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "10px",

            "{name}"
        }
    )
}
