use axum::{
    body::{Body, BoxBody},
    extract::{FromRef, State},
    http::{Request, Response, StatusCode, Uri},
    Router,
};
use desktop_timetable::app::*;
use leptos::{get_configuration, view, LeptosOptions};
use leptos_axum::generate_route_list;
use log::*;
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    leptos_options: LeptosOptions,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(input: &AppState) -> Self {
        input.leptos_options.clone()
    }
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let mut router = Router::new()
        //.leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_handler)
        .with_state(AppState {
            leptos_options: leptos_options.clone(),
        });

    // Add leptos routes. The default implementation (impl LeptosRoutes) currently breaks the type-safe State extractor.
    for path in routes.iter() {
        router = router.route(
            path,
            axum::routing::get(leptos_axum::render_app_to_stream(
                leptos_options.clone(),
                (|cx| view! { cx, <App/> }).clone(),
            )),
        );
    }

    info!("Listening on {}", leptos_options.site_address.clone());
    axum::Server::bind(&leptos_options.site_address)
        .serve(router.into_make_service())
        .await
        .unwrap();
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
            info!("Serving file {}", uri);
            Ok(res.map(axum::body::boxed))
        }
        Err(err) => {
            error!("Error while serving file {}: {:?}", uri, err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_owned()))
        }
    }
}
