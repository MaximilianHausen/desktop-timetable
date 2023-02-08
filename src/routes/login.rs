use leptos::*;

use crate::app::HomeworkerContext;

#[component]
pub fn login_page(cx: Scope) -> impl IntoView {
    // TODO: Maybe move to a custom route in axum and do the redirect checks before handing off to leptos

    // Default used when no context is provided during hydration
    let hw_context = use_context::<HomeworkerContext>(cx).unwrap_or(HomeworkerContext {
        client_id: create_signal(cx, "".to_owned()).0,
        access_token: create_signal(cx, None).0,
        refresh_token: create_signal(cx, None).0,
    });

    let login_status = create_resource(
        cx,
        || (),
        async move |_| {
            let access_token = (hw_context.access_token)();
            if access_token.is_none() {
                return false;
            }
            let response = homeworker::HomeworkerClient::new(
                access_token.unwrap(),
                "desktop-timetable".to_owned(),
            )
            .get_me()
            .await;

            // FIXME: This is NOT a solution (For some reason, homeworker doesn't return all fields with this token,
            // so deserialization errors are ignored)
            response.is_ok()
                || match response.err().unwrap() {
                    homeworker::Error::RequestError(_) => true,
                    homeworker::Error::ApiError(_) => false,
                }
        },
    );

    let login_url = create_resource(
        cx,
        || (),
        async move |_| {
            format!("https://homeworker.li/auth/oauth2/authorize?client_id={}&scopes=me courses.memberships timetable", (hw_context.client_id)())
        },
    );

    view! { cx,
        // TODO: Remove fullscreen div, put classes on body
        <div class="font-rubik dark:bg-zinc-900 dark:text-white w-screen h-screen flex flex-col justify-center items-center">
        <Transition fallback=move || { None::<View> }>
            {move || {
                match login_status.read() {
                    Some(value) => {
                        if value {
                            // Redirect to app
                            Some(view! { cx,
                                <script>
                                    "window.location.href = \"/app\""
                                </script>
                                <meta http-equiv="refresh" content="0; url=/app" />
                                <a href="/app">"Redirect"</a>
                            }.into_view(cx))
                        } else {
                            // Show login page
                            Some(view! { cx,
                                <div class="w-96 p-4 border rounded-xl dark:border-zinc-400">
                                    <h1 class="text-center mb-4">"Anmelden mit Homeworker"</h1>
                                    <p class="mb-4">"Um den Stundenplan abzurufen, musst du dich mit Homeworker anmelden"</p>
                                    <a href=login_url.read()>
                                        <div class="h-9 bg-sky-500 hover:bg-sky-400 rounded-xl flex justify-center items-center">
                                            "Anmelden"
                                        </div>
                                    </a>
                                </div>
                            }.into_view(cx))
                        }
                    },
                    None => {
                        None::<View>
                    }
                }
            }}
        </Transition>
        </div>
    }
}
