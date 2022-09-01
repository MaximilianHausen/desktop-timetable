use dioxus::prelude::*;
use crate::components::timetable::BlockPosition;

pub fn LessonGrid(cx: Scope) -> Element {
    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "flex-start",
            gap: "var(--large-gap-size)",

            LessonColumn {
                lessons: vec![
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP1", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP1", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP1", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(3, "WIP1", None)),
                ]
            }
            LessonColumn {
                lessons: vec![
                    LessonPropsEnum::Multi(MultiLessonProps::new(vec![SingleLessonProps::new(1, "WIP2", None), SingleLessonProps::new(1, "WIP2", None)])),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP2", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP2", None)),
                ]
            }
            LessonColumn {
                lessons: vec![
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP3", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP3", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP3", None)),
                    LessonPropsEnum::Multi(MultiLessonProps::new(vec![SingleLessonProps::new(1, "WIP3", None), SingleLessonProps::new(2, "WIP3", None)])),
                ]
            }
            LessonColumn {
                lessons: vec![
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP4", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP4", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP4", None)),
                ]
            }
            LessonColumn {
                lessons: vec![
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP5", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP5", None)),
                    LessonPropsEnum::Single(SingleLessonProps::new(2, "WIP5", None)),
                ]
            }
        }
    )
}

#[derive(PartialEq, Clone)]
pub enum LessonPropsEnum<'a> {
    Single(SingleLessonProps<'a>),
    Multi(MultiLessonProps<'a>),
}

#[derive(Props, PartialEq, Clone)]
pub struct LessonColumnProps<'a> {
    pub lessons: Vec<LessonPropsEnum<'a>>,
}

pub fn LessonColumn<'a>(cx: Scope<'a, LessonColumnProps<'a>>) -> Element {
    let lessons = cx.props
        .lessons
        .iter()
        .map(|lesson| {
            match lesson {
                LessonPropsEnum::Single(prop) => rsx!(
                    SingleLesson {
                        length: prop.length,
                        name: prop.name,
                        border_style: BlockPosition::Alone,
                    }
                ),
                LessonPropsEnum::Multi(prop) => rsx!(
                    MultiLesson {
                        lessons: prop.lessons.clone(),
                    }
                ),
            }
        });

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

#[derive(Props, PartialEq, Clone)]
pub struct SingleLessonProps<'a> {
    pub length: i8,
    pub name: &'a str,
    pub border_style: Option<BlockPosition>,
}

impl<'a> SingleLessonProps<'a> {
    pub fn new(length: i8, name: &'a str, border_style: Option<BlockPosition>) -> Self {
        Self {
            length,
            name,
            border_style,
        }
    }
}

pub fn SingleLesson<'a>(cx: Scope<'a, SingleLessonProps<'a>>) -> Element {
    let gap = match cx.props.border_style.as_ref().unwrap_or(&BlockPosition::Alone) {
        BlockPosition::Alone => "0px",
        BlockPosition::Top => "(var(--small-gap-size) / 2)",
        BlockPosition::Middle => "var(--small-gap-size)",
        BlockPosition::Bottom => "(var(--small-gap-size) / 2)",
    };
    let radius = match cx.props.border_style.as_ref().unwrap_or(&BlockPosition::Alone) {
        BlockPosition::Alone => "10px",
        BlockPosition::Top => "10px 10px 3px 3px",
        BlockPosition::Middle => "3px 3px 3px 3px",
        BlockPosition::Bottom => "3px 3px 10px 10px",
    };
    rsx!(cx,
        div {
            width: "var(--lesson-size)",
            height: "calc({cx.props.length} * 0.3 * var(--lesson-size) - {gap})",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            outline: "1px solid black",
            border_radius: "{radius}",

            "{cx.props.name}"
        }
    )
}

#[derive(Props, PartialEq, Clone)]
pub struct MultiLessonProps<'a> {
    pub lessons: Vec<SingleLessonProps<'a>>,
}

impl<'a> MultiLessonProps<'a> {
    pub fn new(lessons: Vec<SingleLessonProps<'a>>) -> Self {
        Self { lessons }
    }
}

pub fn MultiLesson<'a>(cx: Scope<'a, MultiLessonProps<'a>>) -> Element {
    let mut counter = 0;
    let lessons = cx.props
        .lessons
        .iter()
        .map(|lesson| {
            let border_style = if cx.props.lessons.len() == 1 {
                BlockPosition::Alone
            } else if counter == 0 {
                BlockPosition::Top
            } else if counter == cx.props.lessons.len() - 1 {
                BlockPosition::Bottom
            } else {
                BlockPosition::Middle
            };

            counter += 1;

            rsx!(cx,
                SingleLesson {
                    length: lesson.length,
                    name: lesson.name,
                    border_style: border_style,
                }
            )
        });

    rsx!(cx,
        div {
            display: "flex",
            flex_direction: "column",
            gap: "var(--small-gap-size)",

            lessons
        }
    )
}
