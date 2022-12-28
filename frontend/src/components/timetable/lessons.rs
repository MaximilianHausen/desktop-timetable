use dioxus::prelude::*;

use crate::components::timetable::BlockPosition;
use crate::types::timetable::Lesson;

#[inline_props]
pub fn LessonColumn(cx: Scope, lesson_groups: Vec<Vec<Option<Lesson>>>) -> Element {
    let lessons = lesson_groups.iter().map(|group| {
        rsx!(LessonGroup {
            lessons: group.clone(),
        })
    });

    rsx!(
        cx,
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            gap: "var(--large-gap-size)",

            lessons
        }
    )
}

#[inline_props]
pub fn LessonGroup(cx: Scope, lessons: Vec<Option<Lesson>>) -> Element {
    // Batch lessons
    let mut batched_lessons: Vec<(Option<&Lesson>, u8)> = vec![];

    for lesson in lessons {
        if !batched_lessons.is_empty() && batched_lessons.last().unwrap().0 == lesson.as_ref() {
            batched_lessons.last_mut().unwrap().1 += 1;
        } else {
            batched_lessons.push((lesson.as_ref(), 1));
        }
    }

    // Render lessons
    let mut lesson_elements: Vec<LazyNodes> = vec![];

    for (i, batched_lesson) in batched_lessons.iter().enumerate() {
        match batched_lesson.0 {
            Some(lesson) => {
                let prev_lesson =
                    (batched_lessons.get(if i > 0 { i - 1 } else { std::usize::MAX }))
                        .and_then(|o| o.0);
                let next_lesson = batched_lessons.get(i + 1).and_then(|o| o.0);

                let border_style = if prev_lesson.is_none() && next_lesson.is_none() {
                    BlockPosition::Alone
                } else if prev_lesson.is_none() && next_lesson.is_some() {
                    BlockPosition::Top
                } else if prev_lesson.is_some() && next_lesson.is_none() {
                    BlockPosition::Bottom
                } else {
                    BlockPosition::Middle
                };

                let adjacent_count = (batched_lessons.len() - 1).clamp(0, 2) as u8;

                lesson_elements.push(rsx!(Lesson {
                    lesson: lesson.clone(),
                    length: batched_lesson.1,
                    border_style: border_style,
                    adjacent_in_group: adjacent_count,
                }));
            }
            None => {
                let adjacent_count = (batched_lessons.len() - 1).clamp(0, 2) as u8;
                lesson_elements.push(rsx!(LessonSpacer {
                    length: batched_lesson.1,
                    adjacent_in_group: adjacent_count,
                }))
            }
        };
    }

    rsx!(
        cx,
        div {
            display: "flex",
            flex_direction: "column",
            gap: "var(--small-gap-size)",

            lesson_elements
        }
    )
}

#[inline_props]
fn Lesson(
    cx: Scope,
    lesson: Lesson,
    length: u8,
    border_style: BlockPosition,
    adjacent_in_group: u8,
) -> Element {
    let gap = match adjacent_in_group {
        0 => "0px",
        1 => "(var(--small-gap-size) / 2)",
        2 => "var(--small-gap-size)",
        _ => "0px",
    };

    let radius = match border_style {
        BlockPosition::Alone => "10px",
        BlockPosition::Top => "10px 10px 3px 3px",
        BlockPosition::Middle => "3px 3px 3px 3px",
        BlockPosition::Bottom => "3px 3px 10px 10px",
    };

    match length {
        1 => rsx!(cx,
            div {
                width: "var(--lesson-size)",
                height: "calc({length} * 0.3 * var(--lesson-size) - {gap})",
                outline: "1px solid black",
                border_radius: "{radius}",
                box_sizing: "border-box",

                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                grid_template_rows: "repeat(1, 1fr)",

                p {
                    style: "place-self: center",
                    margin: "0",
                    grid_area: "1/1",
                    "{lesson.subject.short_name}"
                }
                p {
                    style: "place-self: center",
                    margin: "0",
                    grid_area: "1/2",
                    "{lesson.subject.room}"
                }
            }
        ),
        2 => rsx!(cx,
            div {
                width: "var(--lesson-size)",
                height: "calc({length} * 0.3 * var(--lesson-size) - {gap})",
                outline: "1px solid black",
                border_radius: "{radius}",
                box_sizing: "border-box",

                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                grid_template_rows: "repeat(2, 1fr)",

                p {
                    style: "place-self: center",
                    margin: "0",
                    grid_area: "1/1",
                    "{lesson.subject.short_name}"
                }
                p {
                    style: "place-self: center",
                    margin: "0",
                    grid_area: "1/2",
                    "{lesson.subject.room}"
                }
                p {
                    style: "place-self: center",
                    margin: "0",
                    grid_area: "2/1/2/3",
                    "{lesson.subject.teacher}"
                }
            }
        ),
        _ => panic!(),
    }
}

#[inline_props]
fn LessonSpacer(cx: Scope, length: u8, adjacent_in_group: u8) -> Element {
    let gap = match adjacent_in_group {
        0 => "0px",
        1 => "(var(--small-gap-size) / 2)",
        2 => "var(--small-gap-size)",
        _ => "0px",
    };

    rsx!(
        cx,
        div {
            width: "var(--lesson-size)",
            height: "calc({length} * 0.3 * var(--lesson-size) - {gap})",
        }
    )
}
