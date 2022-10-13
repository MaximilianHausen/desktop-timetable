use dioxus::prelude::*;

use crate::components::timetable::BlockPosition;
use crate::types::timetable::Lesson;

#[inline_props]
pub fn LessonColumn(cx: Scope, lesson_groups: Vec<Vec<Option<Lesson>>>) -> Element {
    let lessons = lesson_groups
        .iter()
        .map(|group| rsx!(
            LessonGroup {
                lessons: group.clone(),
            }
        ));

    rsx!(cx,
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
    let mut batched_lessons: Vec<(&Option<Lesson>, u8)> = vec![];

    for lesson in lessons {
        if batched_lessons.len() > 0 && batched_lessons.last().unwrap().0 == lesson {
            batched_lessons.last_mut().unwrap().1 += 1;
        } else {
            batched_lessons.push((lesson, 1));
        }
    }

    // Render lessons
    let mut lesson_elements: Vec<LazyNodes> = vec![];

    for (i, batched_lesson) in batched_lessons.iter().enumerate() {
        match batched_lesson.0 {
            Some(lesson) => {
                let prev_lesson = (batched_lessons.get(if i > 0 { i - 1 } else { std::usize::MAX })).and_then(|o| o.0.as_ref());
                let next_lesson = batched_lessons.get(i + 1).and_then(|o| o.0.as_ref());

                let border_style = if prev_lesson.is_none() && next_lesson.is_none() {
                    BlockPosition::Alone
                } else if prev_lesson.is_none() && next_lesson.is_some() {
                    BlockPosition::Top
                } else if prev_lesson.is_some() && next_lesson.is_none() {
                    BlockPosition::Bottom
                } else {
                    BlockPosition::Middle
                };

                lesson_elements.push(rsx!(
                    Lesson {
                        lesson: lesson.to_owned(),
                        length: batched_lesson.1,
                        border_style: border_style,
                    }
                ));
            }
            None => lesson_elements.push(rsx!(
                LessonSpacer {
                    length: batched_lesson.1,
                }
            )),
        };
    }


    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "column",
            gap: "var(--small-gap-size)",

            lesson_elements
        }
    )
}

#[inline_props]
fn Lesson(cx: Scope, lesson: Lesson, length: u8, border_style: BlockPosition) -> Element {
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
            width: "var(--lesson-size)",
            height: "calc({length} * 0.3 * var(--lesson-size) - {gap})",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "{radius}",

            "{lesson.subject.short_name}"
        }
    )
}

#[inline_props]
fn LessonSpacer(cx: Scope, length: u8) -> Element {
    rsx!(cx,
        div {
            width: "var(--lesson-size)",
            height: "calc({length} * 0.3 * var(--lesson-size))",
        }
    )
}
