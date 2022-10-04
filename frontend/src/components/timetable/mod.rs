pub mod appointments;
pub mod days;
pub mod lessons;
pub mod times;

use dioxus::prelude::*;
use appointments::*;
use lessons::*;
use crate::types::timetable::*;

#[derive(PartialEq, Clone, Copy)]
pub enum BlockPosition {
    Alone,
    Top,
    Middle,
    Bottom,
}

#[inline_props]
pub fn Timetable(cx: Scope, state: Timetable) -> Element {
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
                times: state.times.clone(),
                /*vec![
                    vec!["8:00 - 8:45".to_owned(), "8:45 - 9:30".to_owned()],
                    vec!["9:45 - 10:30".to_owned(), "10:30 - 11:15".to_owned()],
                    vec!["11:35 - 12:20".to_owned(), "12:20 - 13:05".to_owned()],
                    vec!["13:20 - 14:05".to_owned(), "14:05 - 14:50".to_owned(), "14:50 - 15:35".to_owned()],
                ]*/
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "flex-start",
                gap: "var(--large-gap-size)",

                days::WeekHeader {
                    days: vec![
                        "Monday".to_owned(),
                        "Tuesday".to_owned(),
                        "Wednesday".to_owned(),
                        "Thursday".to_owned(),
                        "Friday".to_owned()
                    ]
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
                lessons::LessonGrid {
                    lesson_columns: vec![
                        vec![
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP1", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP1", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP1", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(3, "WIP1", None)),
                        ],
                        vec![
                            LessonPropsEnum::Multi(MultiLessonProps::new(vec![SingleLessonProps::new(1, "WIP2", None), SingleLessonProps::new(1, "WIP2", None)])),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP2", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP2", None)),
                        ],
                        vec![
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP3", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP3", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP3", None)),
                            LessonPropsEnum::Multi(MultiLessonProps::new(vec![SingleLessonProps::new(1, "WIP3", None), SingleLessonProps::new(2, "WIP3", None)])),
                        ],
                        vec![
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP4", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP4", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP4", None)),
                        ],
                        vec![
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP5", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP5", None)),
                            LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP5", None)),
                        ],
                    ]
                }
            }
        }
    )
}