use axum::{
    body::{Body, BoxBody},
    extract::{FromRef, Query, State},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use axum_extra::extract::{
    cookie::{Cookie, Expiration, Key, SameSite},
    PrivateCookieJar,
};
use desktop_timetable::app::*;
use http::{Request, StatusCode, Uri};
use leptos::{create_signal, get_configuration, provide_context, view, LeptosOptions};
use leptos_axum::{generate_route_list, handle_server_fns};
use log::*;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

#[derive(Clone, FromRef)]
pub struct AppState {
    leptos_options: LeptosOptions,
    cookie_key: Key,
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let mut router = Router::new()
        .route("/auth", get(oauth_token_exchange_handler))
        .route("/serverfn/*path", post(handle_server_fns))
        .fallback(file_handler);

    // Add leptos routes. The default implementation (impl LeptosRoutes) doesn't provide custom scopes.
    for path in routes.iter() {
        router = router.route(path, get(leptos_route_handler));
    }

    info!("Listening on {}", leptos_options.site_address.clone());
    axum::Server::bind(&leptos_options.site_address)
        .serve(
            router
                .with_state(AppState {
                    leptos_options: leptos_options.clone(),
                    cookie_key: Key::from(std::env::var("HW_SECRET").unwrap().as_ref()),
                })
                .into_make_service(),
        )
        .await
        .unwrap();
}

async fn leptos_route_handler(
    State(leptos_options): State<LeptosOptions>,
    mut cookies: PrivateCookieJar,
    request: Request<Body>,
) -> impl IntoResponse {
    if cookies.get("access-token") == None && cookies.get("refresh-token") != None {
        cookies = refresh_token(
            cookies,
            std::env::var("HW_CLIENT_ID").unwrap(),
            std::env::var("HW_SECRET").unwrap(),
        )
        .await;
    }

    let client_id = std::env::var("HW_CLIENT_ID").unwrap();
    let access_token = cookies.get("access-token").map(|c| c.value().to_owned());
    let refresh_token = cookies.get("refresh-token").map(|c| c.value().to_owned());

    let handler = leptos_axum::render_app_to_stream_with_context(
        leptos_options.clone(),
        move |cx| {
            provide_context(
                cx,
                HomeworkerContext {
                    client_id: create_signal(cx, client_id.clone()).0,
                    access_token: create_signal(cx, access_token.clone()).0,
                    refresh_token: create_signal(cx, refresh_token.clone()).0,
                },
            );
        },
        |cx| view! { cx, <App/> },
    );

    (cookies, handler(request).await)
}

async fn refresh_token(
    cookies: PrivateCookieJar,
    client_id: String,
    client_secret: String,
) -> PrivateCookieJar {
    let refresh_token = match cookies.get("refresh-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => {
            return cookies;
        }
    };

    match homeworker::auth::refresh_token(client_id, client_secret, refresh_token).await {
        Ok(response) => cookies.add(
            Cookie::build("access-token", response.access_token.clone())
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Lax)
                .max_age(Duration::seconds(response.expires_in as i64))
                .finish(),
        ),
        Err(e) => {
            log::error!("Token refresh failed: {:?}", e);
            return cookies;
        }
    }
}

#[derive(Deserialize)]
struct OauthExchangeQueryParams {
    code: Option<String>,
}

async fn oauth_token_exchange_handler(
    query: Query<OauthExchangeQueryParams>,
    mut cookies: PrivateCookieJar,
) -> impl IntoResponse {
    let code = match query.0.code {
        Some(c) => c,
        None => return Ok((cookies, Redirect::to("/login"))),
    };

    match homeworker::auth::exchange_token(
        std::env::var("HW_CLIENT_ID").unwrap(),
        std::env::var("HW_SECRET").unwrap(),
        code,
    )
    .await
    {
        Ok(response) => {
            cookies = cookies.add(
                Cookie::build("access-token", response.access_token)
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Lax)
                    .max_age(Duration::seconds(response.expires_in as i64))
                    .finish(),
            );
            cookies = cookies.add(
                Cookie::build("refresh-token", response.refresh_token)
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Lax)
                    .expires(Expiration::from(OffsetDateTime::now_utc() + Duration::days(729)))
                    .finish(),
            );
            Ok((cookies, Redirect::to("/app")))
        }
        Err(error) => {
            match error {
                homeworker::Error::RequestError(err) => {
                    error!("Error while exchanging token: {}", err.to_string())
                }
                homeworker::Error::ApiError(err) => {
                    error!("Error while exchanging token: {}", err.message)
                }
            }
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error while exchanging the token".to_owned()))
        }
    }
}

async fn file_handler(
    State(leptos_options): State<LeptosOptions>,
    uri: Uri,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    match ServeDir::new(&leptos_options.site_root)
        .oneshot(Request::get(uri.clone()).body(Body::empty()).unwrap())
        .await
    {
        Ok(res) => {
            debug!("Serving file {}", uri);
            Ok(res.map(axum::body::boxed))
        }
        Err(err) => {
            error!("Error while serving file {}: {:?}", uri, err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_owned()))
        }
    }
}
