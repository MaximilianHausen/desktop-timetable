use dioxus::prelude::*;
use crate::components::timetable::BlockPosition;

#[inline_props]
pub fn TimeColumn(cx: Scope, times: Vec<Vec<String>>) -> Element {
    let time_column_elements = times
        .iter()
        .map(|time| rsx! {
            MultiTime {
                times: time,
            }
        });

    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            gap: "var(--large-gap-size)",

            time_column_elements
        }
    )
}

#[inline_props]
pub fn SingleTime<'a>(cx: Scope, time: &'a str, border_style: BlockPosition) -> Element {
    let gap = match border_style {
        BlockPosition::Alone => "0px",
        BlockPosition::Top => "(var(--small-gap-size) / 2)",
        BlockPosition::Middle => "var(--small-gap-size)",
        BlockPosition::Bottom => "(var(--small-gap-size) / 2)",
    };

    let radius = match border_style {
        BlockPosition::Alone => "10px",
        BlockPosition::Top => "10px 10px 3px 3px",
        BlockPosition::Middle => "3px 3px 3px 3px",
        BlockPosition::Bottom => "3px 3px 10px 10px",
    };

    rsx!(cx,
        div {
            width: "calc(6 * var(--base-unit))",
            height: "calc(3 * var(--base-unit) - {gap})",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "{radius}",

            "{time}"
        }

    )
}

#[inline_props]
pub fn MultiTime<'a>(cx: Scope, times: &'a Vec<String>) -> Element {
    let mut counter = 0;
    let times = times
        .iter()
        .map(|time| {
            let border_style = if times.len() == 1 {
                BlockPosition::Alone
            } else if counter == 0 {
                BlockPosition::Top
            } else if counter == times.len() - 1 {
                BlockPosition::Bottom
            } else {
                BlockPosition::Middle
            };

            counter += 1;
            rsx!(
                SingleTime {
                    time: time,
                    border_style: border_style,
                }
            )
        });

    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "column",
            gap: "var(--small-gap-size)",

            times
        }
    )
}
