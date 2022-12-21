use dioxus::prelude::*;
use homeworker::Error;

use crate::new_hw_client;
use crate::types::timetable::*;

pub fn DashboardPage(cx: Scope) -> Element {
    let timetable: &UseFuture<Result<Timetable, Error>> = use_future(&cx, (), |_| async move {
        let hw_client = new_hw_client();
        let course_id = hw_client.get_course_memberships().await?[0].course_id;
        let raw_timetable = hw_client.get_timetable(course_id).await?;

        let mut timetable = Timetable {
            times: vec![],
            columns: vec![],
        };

        // Find times (Currently hardcoded)
        timetable
            .times
            .push(vec!["8:00 - 8:45".to_string(), "8:45 - 9:30".to_string()]);
        timetable.times.push(vec![
            "9:45 - 10:30".to_string(),
            "10:30 - 11:15".to_string(),
        ]);
        timetable.times.push(vec![
            "11:35 - 12:20".to_string(),
            "12:20 - 13:05".to_string(),
        ]);
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
                let lesson = Lesson {
                    subject: Subject {
                        full_name: match &raw_lesson.lessons {
                            Some(l) => l.first().unwrap().name.clone(),
                            None => "".to_owned(),
                        },
                        short_name: match &raw_lesson.lessons {
                            Some(l) => l.first().unwrap().short.clone(),
                            None => "".to_owned(),
                        },
                        color: (255, 255, 255),
                    },
                    status: crate::types::timetable::LessonStatus::Normal,
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

        Ok(timetable)
    });

    match timetable.value() {
        Some(Ok(val)) => rsx!(cx,
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "space-between",
                align_items: "center",

                height: "calc(100vh - 16px)",

                // Placeholders for page switching arrows
                div {}

                crate::components::timetable::Timetable {
                    state: val.clone(),
                }

                div {}
            }
        ),
        Some(Err(err)) => match err {
            Error::RequestError(err) => {
                let err_str = err.to_string();
                rsx!(cx, "{err_str}")
            }
            Error::ApiError(err) => match err.code {
                401 => rsx!(cx, Redirect { to: "/login" }),
                _ => rsx!(cx, "Fehler beim Überprüfen des Anmeldestatus"),
            },
        },
        None => None,
    }
}
