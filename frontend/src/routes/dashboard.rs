use dioxus::prelude::*;
use homeworker::Error;

use crate::new_hw_client;
use crate::types::timetable::Timetable;

pub fn DashboardPage(cx: Scope) -> Element {
    let timetable: &UseFuture<Result<Timetable, Error>> = use_future(&cx, (), |_| async move {
        let raw_timetable = new_hw_client().get_timetable(0).await?;
        //TODO: Timetable format conversion
        Ok(Timetable { times: vec![], columns: vec![] })
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
        Some(Err(err)) => {
            match err {
                Error::RequestError(err) => {
                    let err_str = err.to_string();
                    rsx!(cx, "{err_str}")
                },
                Error::ApiError(err) => {
                    match err.code {
                        401 => rsx!(cx, Redirect { to: "/login" }),
                        _ => rsx!(cx, "Fehler beim Überprüfen des Anmeldestatus")
                    }
                }
            }
        },
        None => None,
    }
}
