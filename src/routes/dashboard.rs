use leptos::*;

use crate::{app::HomeworkerContext, types::timetable::*};

#[component]
pub fn dashboard_page(cx: Scope) -> impl IntoView {
    // Default used when no context is provided during hydration
    let hw_context = use_context::<HomeworkerContext>(cx).unwrap_or(HomeworkerContext {
        client_id: create_signal(cx, "".to_owned()).0,
        access_token: create_signal(cx, None).0,
        refresh_token: create_signal(cx, None).0,
    });

    let timetable_resource = create_resource(
        cx,
        || (),
        async move |_| -> Option<Timetable> {
            let access_token = (hw_context.access_token)();
            if access_token.is_none() {
                return None;
            }

            let client = homeworker::HomeworkerClient::new(
                access_token.unwrap(),
                "desktop-timetable".to_owned(),
            );

            let course_id = client.get_course_memberships().await.unwrap()[0].course_id;
            let raw_timetable = client.get_timetable(course_id).await.unwrap();

            let mut timetable = Timetable {
                times: vec![],
                columns: vec![],
            };

            // Find times (Currently hardcoded)
            timetable
                .times
                .push(vec!["8:00 - 8:45".to_string(), "8:45 - 9:30".to_string()]);
            timetable
                .times
                .push(vec!["9:45 - 10:30".to_string(), "10:30 - 11:15".to_string()]);
            timetable
                .times
                .push(vec!["11:35 - 12:20".to_string(), "12:20 - 13:05".to_string()]);
            timetable.times.push(vec![
                "13:20 - 14:05".to_string(),
                "14:05 - 14:50".to_string(),
                "14:05 - 15:35".to_string(),
            ]);

            // List lessons
            for day in raw_timetable.iter().take(5) {
                let mut last_position = 0;
                let mut lessons: Vec<Option<Lesson>> = vec![];

                for raw_lesson in day.lessons.iter().filter(|l| !l.is_break) {
                    let lesson = {
                        let raw_lesson = raw_lesson.lessons.as_ref().and_then(|l| l.first());
                        Lesson {
                            subject: Subject {
                                full_name: match raw_lesson {
                                    Some(l) => l.name.clone(),
                                    None => "".to_owned(),
                                },
                                short_name: match raw_lesson {
                                    Some(l) => l.short.clone(),
                                    None => "".to_owned(),
                                },
                                teacher: match raw_lesson {
                                    Some(l) => l.teacher.clone(),
                                    None => "".to_owned(),
                                },
                                room: match raw_lesson {
                                    Some(l) => l.room.clone(),
                                    None => "".to_owned(),
                                },
                                color: (255, 255, 255),
                            },
                            status: crate::types::timetable::LessonStatus::Normal,
                        }
                    };

                    // Fill empty lessons before
                    (last_position..*raw_lesson.unit.positions.first().unwrap() - 1)
                        .for_each(|_| lessons.push(None));
                    last_position = *raw_lesson.unit.positions.last().unwrap();

                    // Add this lesson once for each position it occupies
                    (0..raw_lesson.unit.positions.len())
                        .for_each(|_| lessons.push(Some(lesson.clone())));
                }

                timetable.columns.push(TimetableColumn {
                    name: day.date.weekday().to_string(),
                    lessons,
                });
            }

            Some(timetable)
        },
    );

    // Name overlap with types::timetable::Timetable and components::timetable::Timetable
    use crate::components::timetable::*;

    view! { cx,
        // TODO: Remove fullscreen div, put classes on body
        <div class="font-rubik dark:bg-zinc-900 dark:text-white w-screen h-screen flex flex-col justify-center items-center">
        <Transition fallback=move || { None::<View> }>
            {move || {
                match timetable_resource.read() {
                    Some(Some(timetable)) => Some(view! { cx, <Timetable state=timetable/> }.into_view(cx)),
                    _ => None::<View>,
                }
            }}
        </Transition>
        </div>
    }
}
