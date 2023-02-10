use leptos::*;

use crate::types::timetable::{GroupedTimetableColumn, Lesson, Timetable};

#[derive(PartialEq, Clone, Copy)]
pub enum BlockPosition {
    Alone,
    Top,
    Middle,
    Bottom,
}

#[component]
pub fn timetable(cx: Scope, state: Timetable) -> impl IntoView {
    let (timetable, _) = create_signal(cx, state);

    let column_names = move || -> Vec<String> {
        timetable()
            .columns
            .iter()
            .map(|column| column.name.clone())
            .collect()
    };

    let time_groups = move || timetable().times;

    let lesson_group_sizes = move || {
        time_groups()
            .iter()
            .map(|vec| vec.len())
            .collect::<Vec<usize>>()
    };

    let grouped_lesson_columns = move || {
        timetable()
            .columns
            .into_iter()
            .map(|column| {
                let mut groups: Vec<Vec<Option<Lesson>>> = vec![];

                let mut pos = 0;
                for group_size in lesson_group_sizes() {
                    if pos >= column.lessons.len() {
                        break;
                    } else if pos + group_size >= column.lessons.len() {
                        groups.push(column.lessons[pos..column.lessons.len()].to_vec());
                        break;
                    } else {
                        groups.push(column.lessons[pos..(pos + group_size)].to_vec());
                        pos += group_size;
                    }
                }

                GroupedTimetableColumn {
                    name: column.name,
                    lessons: groups,
                }
            })
            .collect::<Vec<GroupedTimetableColumn>>()
    };

    view! { cx,
        <div class="grid gap-4">
            // ========== Days ==========
            <div class="row-start-1 col-start-2 row-end-1 col-end-2 flex gap-4">
                <For
                    each=column_names
                    key=|s| s.clone()
                    view=move |name: String| view! { cx,
                        <div class="w-44 h-10 flex justify-center items-center border border-zinc-400 rounded-lg">
                            {name}
                        </div>
                    }
                />
            </div>
            // ========== Appointments ==========
            <div class="row-start-2 col-start-2 row-end-2 col-end-2 h-8 bg-orange-500">
            </div>
            // ========== Times ==========
            <div class="row-start-3 col-start-1 row-end-3 col-end-1 flex flex-col gap-4">
                <For
                    each=time_groups
                    key=|g| g.clone()
                    view=move |group| {
                        let mut counter = 0;
                        let items: Vec<_> = group.iter().map(|time| {
                            let border_style = if group.len() == 1 {
                                BlockPosition::Alone
                            } else if counter == 0 {
                                BlockPosition::Top
                            } else if counter == group.len() - 1 {
                                BlockPosition::Bottom
                            } else {
                                BlockPosition::Middle
                            };

                            let height = match border_style {
                                BlockPosition::Alone => "h-14",
                                BlockPosition::Top => "h-[3.375rem]",
                                BlockPosition::Middle => "h-[3.25rem]",
                                BlockPosition::Bottom => "h-[3.375rem]",
                            };

                            let border = match border_style {
                                BlockPosition::Alone => "rounded-lg",
                                BlockPosition::Top => "rounded-t-lg rounded-b-sm",
                                BlockPosition::Middle => "rounded-sm",
                                BlockPosition::Bottom => "rounded-t-sm rounded-b-lg",
                            };

                            counter += 1;

                            let class = format!("min-w-[6rem] {height} p-3 flex justify-center items-center border border-zinc-400 {border}");
                            view! { cx,
                                <div class=class>
                                    {time}
                                </div>
                            }
                        }).collect();

                        view! { cx,
                            <div class="flex flex-col gap-1">
                                {items}
                            </div>
                        }
                    }
                />
            </div>
            // ========== Lessons ==========
            <div class="row-start-3 col-start-2 row-end-3 col-end-2 flex items-start gap-4">
                <For
                    each=grouped_lesson_columns
                    key=|c| c.name.clone()
                    view=move |column| {
                        let lesson_group_elements: Vec<_> = column.lessons.iter().map(|group| {
                            let mut batched_lessons: Vec<(Option<&Lesson>, u8)> = vec![];

                            for lesson in group {
                                if !batched_lessons.is_empty() && batched_lessons.last().unwrap().0 == lesson.as_ref() {
                                    batched_lessons.last_mut().unwrap().1 += 1;
                                } else {
                                    batched_lessons.push((lesson.as_ref(), 1));
                                }
                            }

                            let mut lesson_elements: Vec<View> = vec![];

                            for (i, batched_lesson) in batched_lessons.iter().enumerate() {
                                match batched_lesson.0 {
                                    Some(lesson) => {
                                        let prev_lesson =
                                            (batched_lessons.get(if i > 0 { i - 1 } else { std::usize::MAX }))
                                                .and_then(|o| o.0);
                                        let next_lesson = batched_lessons.get(i + 1).and_then(|o| o.0);

                                        let border = if prev_lesson.is_none() && next_lesson.is_none() {
                                            BlockPosition::Alone
                                        } else if prev_lesson.is_none() && next_lesson.is_some() {
                                            BlockPosition::Top
                                        } else if prev_lesson.is_some() && next_lesson.is_none() {
                                            BlockPosition::Bottom
                                        } else {
                                            BlockPosition::Middle
                                        };

                                        let adjacent_count = (batched_lessons.len() - 1).clamp(0, 2) as u8;

                                        lesson_elements.push(view! { cx,
                                            <Lesson lesson=lesson.clone() length=batched_lesson.1 border=border adjacent_count=adjacent_count/>
                                        }.into_view(cx));
                                    }
                                    None => {
                                        let adjacent_count = (batched_lessons.len() - 1).clamp(0, 2) as u8;
                                        let gap = match adjacent_count {
                                            0 => "0rem",
                                            1 => "0.125rem",
                                            2 => "0.25rem",
                                            _ => "0rem",
                                        };
                                        lesson_elements.push(view! { cx,
                                            <div style=format!("height: calc({} * 3.5rem - {});", batched_lesson.1, gap) class="w-44"/>
                                        }.into_view(cx));
                                    }
                                };
                            }

                            view! { cx,
                                <div class="flex flex-col gap-1">
                                    {lesson_elements}
                                </div>
                            }
                        }).collect();

                        view! { cx,
                            <div class="flex flex-col gap-4">
                                {lesson_group_elements}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn lesson(
    cx: Scope,
    lesson: Lesson,
    length: u8,
    border: BlockPosition,
    adjacent_count: u8,
) -> impl IntoView {
    let gap = match adjacent_count {
        0 => "0rem",
        1 => "0.125rem",
        2 => "0.25rem",
        _ => "0rem",
    };

    let border_class = match border {
        BlockPosition::Alone => "rounded-lg",
        BlockPosition::Top => "rounded-t-lg rounded-b-sm",
        BlockPosition::Middle => "rounded-sm",
        BlockPosition::Bottom => "rounded-t-sm rounded-b-lg",
    };

    let common_class = format!("w-44 border border-zinc-400 {border_class}");

    match length {
        1 => view! { cx,
            <div style=format!("height: calc({length} * 3.5rem - {gap});") class=format!("{common_class} grid grid-cols-2 place-items-center")>
                <p>{lesson.subject.short_name}</p>
                <p>{lesson.subject.room}</p>
            </div>
        },
        2 => view! { cx,
            <div style=format!("height: calc({length} * 3.5rem - {gap});") class=format!("{common_class} grid grid-cols-2 grid-rows-2 place-items-center")>
                <p>{lesson.subject.short_name}</p>
                <p>{lesson.subject.room}</p>
                <p class="col-span-full">{lesson.subject.teacher}</p>
            </div>
        },
        _ => panic!("Unecpected lesson length: {length}. Lesson lengths must be either 1 or 2"),
    }
}
