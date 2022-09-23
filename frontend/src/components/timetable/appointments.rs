use dioxus::prelude::*;

#[inline_props]
pub fn AppointmentBar<'a>(cx: Scope, appointment_lines: Vec<Vec<AppointmentPropsEnum<'a>>>) -> Element {
    let appointment_bar_elements = appointment_lines
        .iter()
        .map(|appointment_line| rsx! {
            AppointmentLine {
                appointments: appointment_line,
            }
        });

    rsx! {cx,
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "flex-start",
            align_items: "center",
            gap: "var(--small-gap-size)",

            appointment_bar_elements
        }
    }
}

pub enum AppointmentPropsEnum<'a> {
    Appointment(AppointmentProps<'a>),
    Spacer(AppointmentSpacerProps),
}

#[inline_props]
pub fn AppointmentLine<'a>(cx: Scope, appointments: &'a Vec<AppointmentPropsEnum<'a>>) -> Element {
    let appointment_elements = cx.props.appointments
        .iter()
        .map(|appointment| match appointment {
            AppointmentPropsEnum::Appointment(prop) => rsx! {
                Appointment {
                    length: prop.length,
                    name: prop.name,
                }
            },
            AppointmentPropsEnum::Spacer(prop) => rsx! {
                AppointmentSpacer {
                    length: prop.length,
                }
            },
        });

    rsx! {cx,
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "flex-start",
            align_items: "center",
            gap: "var(--large-gap-size)",

            appointment_elements,
        }
    }
}

#[inline_props]
pub fn Appointment<'a>(cx: Scope, length: u8, name: &'a str) -> Element {
    let gap_count = length - 1;
    rsx! {cx,
        div {
            width: "calc(var(--lesson-size) * {length} + var(--large-gap-size) * {gap_count})",
            height: "calc(1 * var(--base-unit))",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "5px",

            "{name}"
        }
    }
}

#[inline_props]
pub fn AppointmentSpacer(cx: Scope, length: u8) -> Element {
    let gap_count = length - 1;
    rsx!(cx,
        div {
            width: "calc(var(--lesson-size) * {length} + var(--large-gap-size) * {gap_count})",
            height: "calc(1 * var(--base-unit))",
        }
    )
}
