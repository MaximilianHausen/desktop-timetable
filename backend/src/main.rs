use axum::{Extension, Router};
use axum::http::StatusCode;
use axum::routing::post;
use axum_extra::extract::cookie::{Cookie, Key};
use axum_extra::extract::PrivateCookieJar;

#[tokio::main]
async fn main() {
    let key = Key::from(std::env::var("CLIENT_SECRET").unwrap().as_ref());

    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .layer(Extension(key));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login(mut jar: PrivateCookieJar, body: String) -> Result<PrivateCookieJar, StatusCode> {
    match homeworker_api_client::auth::exchange_token(std::env::var("CLIENT_ID").unwrap(), std::env::var("CLIENT_SECRET").unwrap(), body).await {
        Ok(response) => {
            jar = jar.add(Cookie::build("proxy-session", format!("{}|{}", response.access_token, response.refresh_token)).http_only(true).secure(true).finish());
            Ok(jar)
        }
        Err(error) => match error {
            homeworker_api_client::Error::RequestError(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            homeworker_api_client::Error::ApiError(err) => Err(StatusCode::from_u16(err.code).unwrap()),
        },
    }
}

async fn logout(jar: PrivateCookieJar) -> PrivateCookieJar {
    jar.remove(Cookie::named("proxy-session"))
}
