use dioxus::prelude::*;
use homeworker::Error;
use homeworker::types::users::User;

use crate::{HW_CLIENT_ID, new_hw_client};

pub fn LoginPage(cx: Scope) -> Element {
    let user: &UseFuture<Result<User, Error>> = use_future(&cx, (), |(_)| async move {
        new_hw_client().get_me().await
    });

    match user.value() {
        Some(Ok(_)) => rsx!(cx, Redirect { to: "/app" }),
        Some(Err(err)) => {
            match err {
                Error::RequestError(err) => {
                    let err_str = err.to_string();
                    rsx!(cx, "Reqwest error: {err_str}")
                },
                Error::ApiError(err) => {
                    match err.code {
                        401 => rsx!(cx,
                            div {
                                position: "absolute",
                                top: "50%",
                                left: "50%",
                                transform: "translate(-50%,-50%)",

                                p { "Um den Stundenplan abzurufen, musst du dich mit Homeworker anmelden" }
                                a {
                                    href: "https://homeworker.li/auth/oauth2/authorize?client_id={HW_CLIENT_ID}&scopes=me courses.memberships timetable",
                                    "Login"
                                }
                            }
                        ),
                        _ => rsx!(cx, "Fehler beim Überprüfen des Anmeldestatus")
                    }
                }
            }
        },
        None => None,
    }
}
