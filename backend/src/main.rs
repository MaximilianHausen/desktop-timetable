use std::net::SocketAddr;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::http::StatusCode;
use axum::Router;
use axum::routing::post;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use homeworker_api_client::Error;
use rand::distributions::{Alphanumeric, DistString};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct Tokens {
    refresh_token: String,
    access_token: String,
    expires_at: u64,
}

async fn login(mut jar: CookieJar, body: String) -> Result<CookieJar,StatusCode> {
    // Handle existing cookie
    if let Some(cookie) = jar.get("proxy-session") {
        //TODO: Check cookie for only alphanumerics
        if !db_query(format!("SELECT * FROM session:{};", cookie.value())).await[0].result.is_empty() {
            // Already authorized
            return Ok(jar);
        } else {
            jar = jar.remove(Cookie::named("proxy-session"));
        }
    }

    let tokens = match homeworker_api_client::auth::exchange_token(std::env::var("HW_ID").unwrap(), std::env::var("HW_SECRET").unwrap(), body).await {
        Ok(response) => Tokens {
            refresh_token: response.refresh_token,
            access_token: response.access_token,
            expires_at: (SystemTime::now() + Duration::from_secs(response.expires_in as u64)).duration_since(UNIX_EPOCH).unwrap().as_secs(),
        },
        Err(error) => return match error {
            Error::RequestError(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Error::ApiError(err) => Err(StatusCode::from_u16(err.code).unwrap()),
        },
    };

    let session: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 50);

    let query = format!("CREATE session:{} SET refresh_token = {}, access_token = {}, expires_at = {};", &session, tokens.refresh_token, tokens.access_token, tokens.expires_at);
    if db_query(query).await[0].result.is_empty() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    jar = jar.add(Cookie::build("proxy-session", session).http_only(true).secure(true).finish());
    Ok(jar)
}

async fn logout(mut jar: CookieJar) -> Result<CookieJar, (CookieJar, StatusCode)> {
    // Get session cookie
    let session = match jar.get("proxy-session") {
        Some(cookie) => cookie.value().to_owned(),
        None => return Err((jar, StatusCode::UNAUTHORIZED)),
    };

    jar = jar.remove(Cookie::named("proxy-session"));
    // Close session
    //TODO: Check cookie for only alphanumerics
    if db_query(format!("DELETE session:{} RETURN BEFORE;", session)).await[0].result.is_empty() {
        return Err((jar, StatusCode::UNAUTHORIZED));
    }
    Ok(jar)
}

#[derive(serde::Deserialize)]
struct DbResponse {
    time: String,
    status: String,
    result: Vec<serde_json::Value>,
}

async fn db_query(query: String) -> Vec<DbResponse> {
    let response = reqwest::Client::new().post(std::env::var("DB_URL").unwrap())
        .basic_auth(std::env::var("DB_USER").unwrap(), std::env::var("DB_PASS").ok())
        .header("NS", "hwproxy")
        .header("DB", "hwproxy")
        .header("Content-Type", "application/json")
        .body(query)
        .send().await.unwrap();

    serde_json::from_str(&response.text().await.unwrap()).unwrap()
}
