use dioxus::prelude::*;

use appointments::*;
use lessons::*;

use crate::types::timetable::*;

pub mod appointments;
pub mod days;
pub mod lessons;
pub mod times;

#[derive(PartialEq, Clone, Copy)]
pub enum BlockPosition {
    Alone,
    Top,
    Middle,
    Bottom,
}

#[inline_props]
pub fn Timetable(cx: Scope, state: Timetable) -> Element {
    let lesson_groups: Vec<usize> = state.times.iter().map(|vec| vec.len()).collect();

    let times = state.times.clone();
    let column_names: Vec<String> = state.columns.iter().map(|column| column.name.clone()).collect();

    let lesson_columns = state.columns.iter().map(|column| {
        let mut groups: Vec<Vec<Option<Lesson>>> = vec![];

        let mut pos = 0;
        for group_size in &lesson_groups {
            if pos >= column.lessons.len() {
                break;
            } else if pos + group_size >= column.lessons.len() {
                let mut last_group = column.lessons[pos..column.lessons.len()].to_vec();
                // Fill empty trailing lessons
                for _ in column.lessons.len()..pos + group_size {
                    last_group.push(None);
                }
                groups.push(last_group);
                break;
            } else {
                groups.push(column.lessons[pos..(pos + group_size)].to_vec());
                pos += group_size;
            }
        }

        rsx!(
            LessonColumn {
                lesson_groups: groups,
            }
        )
    });

    rsx!(cx,
        div {
            style: "
                --base-unit: 2vh;
                --large-gap-size: calc(1 * var(--base-unit));
                --medium-gap-size: calc(0.5 * var(--base-unit));
                --small-gap-size: calc(0.3 * var(--base-unit));
                --lesson-size: calc(10 * var(--base-unit));",

            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "flex-end",
            gap: "var(--large-gap-size)",

            times::TimeColumn {
                times: times,
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "flex-start",
                gap: "var(--large-gap-size)",

                days::WeekHeader {
                    days: column_names,
                }
                appointments::AppointmentBar {
                    appointment_lines: vec![
                        vec![
                            AppointmentPropsEnum::Spacer(AppointmentSpacerProps {length: 2}),
                            AppointmentPropsEnum::Appointment(AppointmentProps {length: 2, name: "Appointment"})
                        ],
                        vec![
                            AppointmentPropsEnum::Appointment(AppointmentProps {length: 1, name: "Appointment"}),
                            AppointmentPropsEnum::Spacer(AppointmentSpacerProps {length: 2}),
                            AppointmentPropsEnum::Appointment(AppointmentProps {length: 1, name: "Appointment"})
                        ]
                    ]
                }
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "center",
                    align_items: "flex-start",
                    gap: "var(--large-gap-size)",

                    lesson_columns
                }
            }
        }
    )
}
