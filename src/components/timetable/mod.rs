use leptos::*;

use crate::types::timetable::Timetable;

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

    view! { cx,
        <div class="grid gap-4">
            // ========== Days ==========
            <div class="row-start-1 col-start-2 row-end-1 col-end-2 flex gap-4">
                <For
                    each=column_names
                    key=|s| s.clone()
                    view=move |name: String| view! { cx,
                        <div class="w-40 h-10 flex justify-center items-center border border-zinc-400 rounded-lg">
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
            </div>
        </div>
    }
}
