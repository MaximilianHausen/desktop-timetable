use cfg_if::cfg_if;
use leptos::*;

#[cfg(feature = "ssr")]
use crate::app::HomeworkerContext;

#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let hw_context = use_context::<HomeworkerContext>(cx).unwrap();

            let login_status = create_resource(
                cx,
                || (),
                async move |_| {
                    homeworker::HomeworkerClient::new(
                        (hw_context.client_id)(),
                        "desktop-timetable".to_owned(),
                    )
                    .get_me()
                    .await
                    .is_ok()
                },
            );

            view! { cx,
                <Transition fallback=move || { None::<View> }>
                    {move || {
                        match login_status.read() {
                            Some(value) => {
                                if value {
                                    // TODO: Cunfigurable base URL
                                    // Redirect to app
                                    Some(view! { cx,
                                        <script>
                                            "window.location.href = \"https://example.com\""
                                        </script>
                                        <meta http-equiv="refresh" content="0; url=https://example.com/" />
                                        <a href="https://example.com">"Redirect"</a>
                                    }.into_view(cx))
                                } else {
                                    // Show login page
                                    Some(view! { cx,
                                        <div class="font-rubik dark:bg-zinc-900 dark:text-white w-screen h-screen flex flex-col justify-center items-center">
                                            <div class="w-96 p-4 border rounded-xl dark:border-zinc-400">
                                                <h1 class="text-center mb-4">"Anmelden mit Homeworker"</h1>
                                                <p class="mb-4">"Um den Stundenplan abzurufen, musst du dich mit Homeworker anmelden"</p>
                                                <a href={format!("https://homeworker.li/auth/oauth2/authorize?client_id={}&scopes=me courses.memberships timetable", (hw_context.client_id)())}>
                                                    <div class="h-9 bg-sky-500 hover:bg-sky-400 rounded-xl flex justify-center items-center">
                                                        "Anmelden"
                                                    </div>
                                                </a>
                                            </div>
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
            }
        } else {
            None::<View>
        }
    }
}
