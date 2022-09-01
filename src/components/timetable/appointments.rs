use dioxus::prelude::*;

pub fn AppointmentBar(cx: Scope) -> Element {
    rsx! {cx,
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "flex-start",
            align_items: "center",
            gap: "var(--small-gap-size)",

            AppointmentLine {
                appointments: vec![
                    AppointmentPropsEnum::Spacer(AppointmentSpacerProps {length: 2}),
                    AppointmentPropsEnum::Appointment(AppointmentProps {length: 2, name: "Appointment"})
                ],
            }
            AppointmentLine {
                appointments: vec![
                    AppointmentPropsEnum::Appointment(AppointmentProps {length: 1, name: "Appointment"}),
                    AppointmentPropsEnum::Spacer(AppointmentSpacerProps {length: 2}),
                    AppointmentPropsEnum::Appointment(AppointmentProps {length: 1, name: "Appointment"})
                ]
            }
        }
    }
}

#[derive(PartialEq)]
pub enum AppointmentPropsEnum<'a> {
    Appointment(AppointmentProps<'a>),
    Spacer(AppointmentSpacerProps),
}

#[inline_props]
pub fn AppointmentLine<'a>(cx: Scope, appointments: Vec<AppointmentPropsEnum<'a>>) -> Element {
    let appointment_elements = appointments
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

#[derive(Props, PartialEq)]
pub struct AppointmentProps<'a> {
    length: i8,
    name: &'a str,
}

pub fn Appointment<'a>(cx: Scope<'a, AppointmentProps<'a>>) -> Element {
    let gap_count = cx.props.length - 1;
    rsx! {cx,
        div {
            width: "calc(var(--lesson-size) * {cx.props.length} + var(--large-gap-size) * {gap_count})",
            height: "calc(1 * var(--base-unit))",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "5px",

            "{cx.props.name}"
        }
    }
}

#[derive(Props, PartialEq)]
pub struct AppointmentSpacerProps {
    length: i8,
}

pub fn AppointmentSpacer(cx: Scope<AppointmentSpacerProps>) -> Element {
    let gap_count = cx.props.length - 1;
    rsx! (cx,
        div {
            width: "calc(var(--lesson-size) * {cx.props.length} + var(--large-gap-size) * {gap_count})",
            height: "calc(1 * var(--base-unit))",
        }
    )
}
