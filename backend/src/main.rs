use axum::{Extension, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_extra::extract::cookie::{Cookie, Expiration, Key};
use axum_extra::extract::PrivateCookieJar;
use time::{Duration, OffsetDateTime};

#[tokio::main]
async fn main() {
    let key = Key::from(std::env::var("CLIENT_SECRET").unwrap().as_ref());

    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/api/v2/*path", get(proxy_get).post(proxy_post).delete(proxy_delete))
        .layer(Extension(key));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login(mut jar: PrivateCookieJar, body: String) -> Result<PrivateCookieJar, StatusCode> {
    match homeworker::auth::exchange_token(std::env::var("CLIENT_ID").unwrap(), std::env::var("CLIENT_SECRET").unwrap(), body).await {
        Ok(response) => {
            jar = jar.add(Cookie::build("access-token", response.access_token)
                .http_only(true).secure(true)
                .expires(Expiration::DateTime(OffsetDateTime::now_utc() + Duration::seconds(response.expires_in as i64))).finish());
            jar = jar.add(Cookie::build("refresh-token", response.refresh_token)
                .http_only(true).secure(true)
                .expires(Expiration::from(OffsetDateTime::now_utc() + Duration::days(729))).finish());
            Ok(jar)
        }
        Err(error) => match error {
            homeworker::Error::RequestError(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            homeworker::Error::ApiError(err) => Err(StatusCode::from_u16(err.code).unwrap()),
        },
    }
}

async fn logout(jar: PrivateCookieJar) -> PrivateCookieJar {
    jar.remove(Cookie::named("access-token")).remove(Cookie::named("refresh-token"))
}

async fn proxy_get(jar: PrivateCookieJar, Path(path): Path<String>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let access_token = match jar.get("access-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => return Err((StatusCode::UNAUTHORIZED, "".to_owned())),
    };

    let response = match reqwest::Client::new().get("https://homeworker.li/api/v2".to_owned() + &path)
        .header("Authorization", "Bearer ".to_owned() + &access_token)
        .send().await {
        Ok(response) => response,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())),
    };

    Ok((response.status(), response.text().await.unwrap()))
}

async fn proxy_post(jar: PrivateCookieJar, Path(path): Path<String>, body: String) -> Result<(StatusCode, String), (StatusCode, String)> {
    let access_token = match jar.get("access-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => return Err((StatusCode::UNAUTHORIZED, "".to_owned())),
    };

    let response = match reqwest::Client::new().post("https://homeworker.li/api/v2".to_owned() + &path)
        .header("Authorization", "Bearer ".to_owned() + &access_token)
        .body(body)
        .send().await {
        Ok(response) => response,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())),
    };

    Ok((response.status(), response.text().await.unwrap()))
}

async fn proxy_delete(jar: PrivateCookieJar, Path(path): Path<String>, body: String) -> Result<(StatusCode, String), (StatusCode, String)> {
    let access_token = match jar.get("access-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => return Err((StatusCode::UNAUTHORIZED, "".to_owned())),
    };

    let response = match reqwest::Client::new().delete("https://homeworker.li/api/v2".to_owned() + &path)
        .header("Authorization", "Bearer ".to_owned() + &access_token)
        .body(body)
        .send().await {
        Ok(response) => response,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())),
    };

    Ok((response.status(), response.text().await.unwrap()))
}
