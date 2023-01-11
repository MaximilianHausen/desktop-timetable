use std::env::var;

use axum::extract::{FromRef, Path};
use axum::http::header::HeaderMap;
use axum::http::{Method, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::{Cookie, Expiration, Key, SameSite};
use axum_extra::extract::PrivateCookieJar;
use time::{Duration, OffsetDateTime};

struct AppState {
    // Encryption key for the PrivateCookieJar
    key: Key,
}

// this impl tells PrivateCookieJar how to access the key
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() {
    println!(
        "Running for client id {}",
        var("CLIENT_ID").unwrap()
    );
    let key = Key::from(var("CLIENT_SECRET").unwrap().as_ref());

    let app = Router::new()
        .route("/homeworker/login", post(login))
        .route("/homeworker/logout", post(logout))
        .route("/homeworker/*path", get(proxy).post(proxy).delete(proxy))
        .with_state(key);

    let addr = std::net::SocketAddr::new(var("BIND_ADDR").unwrap_or("127.0.0.1".to_owned()).parse().unwrap(), 3000);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login(
    mut cookie_jar: PrivateCookieJar,
    body: String,
) -> Result<PrivateCookieJar, (StatusCode, Json<homeworker::types::ErrorResponse>)> {
    println!("Login with code {body}");

    match homeworker::auth::exchange_token(
        std::env::var("CLIENT_ID").unwrap(),
        std::env::var("CLIENT_SECRET").unwrap(),
        body,
    )
    .await
    {
        Ok(response) => {
            cookie_jar = cookie_jar.add(
                Cookie::build("access-token", response.access_token)
                    .http_only(true)
                    .secure(true)
                    .path("/homeworker")
                    .same_site(SameSite::Lax)
                    .max_age(Duration::seconds(response.expires_in as i64))
                    .finish(),
            );
            cookie_jar = cookie_jar.add(
                Cookie::build("refresh-token", response.refresh_token)
                    .http_only(true)
                    .secure(true)
                    .path("/homeworker")
                    .same_site(SameSite::Lax)
                    .expires(Expiration::from(
                        OffsetDateTime::now_utc() + Duration::days(729),
                    ))
                    .finish(),
            );

            match refresh_auth_key(cookie_jar).await {
                Ok((auth_key, new_jar)) => {
                    cookie_jar = new_jar.add(
                        Cookie::build("auth-key", auth_key)
                            .http_only(true)
                            .secure(true)
                            .path("/homeworker")
                            .same_site(SameSite::Lax)
                            .expires(Expiration::from(
                                OffsetDateTime::now_utc() + Duration::days(729),
                            ))
                            .finish(),
                    )
                }
                Err((error_message, _)) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            homeworker::types::Error {
                                name: "Proxy".to_owned(),
                                message: error_message,
                                code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                            }
                            .into(),
                        ),
                    ));
                }
            }
            Ok(cookie_jar)
        }
        Err(error) => match error {
            homeworker::Error::RequestError(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    homeworker::types::Error {
                        name: "Proxy".to_owned(),
                        message: "Error while exchanging the token".to_owned(),
                        code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                    }
                    .into(),
                ),
            )),
            homeworker::Error::ApiError(err) => Err((StatusCode::BAD_GATEWAY, Json(err.into()))),
        },
    }
}

async fn logout(jar: PrivateCookieJar) -> PrivateCookieJar {
    jar.remove(Cookie::named("access-token"))
        .remove(Cookie::named("refresh-token"))
        .remove(Cookie::named("auth-key"))
}

async fn proxy(
    mut cookie_jar: PrivateCookieJar,
    Path(path): Path<String>,
    method: axum::http::Method,
    headers: HeaderMap,
    body: String,
) -> (
    StatusCode,
    PrivateCookieJar,
    Result<String, Json<homeworker::types::ErrorResponse>>,
) {
    println!("Proxy to {method} {path}");

    let access_token = match cookie_jar.get("access-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => match refresh_token(cookie_jar).await {
            Ok(result) => {
                cookie_jar = result.1;
                result.0
            }
            Err(err) => {
                cookie_jar = err.1;
                return (
                    StatusCode::UNAUTHORIZED,
                    cookie_jar,
                    Err(Json(new_error(StatusCode::UNAUTHORIZED, err.0).into())),
                );
            }
        },
    };

    let auth_key = match cookie_jar.get("auth-key") {
        Some(cookie) => cookie.value().to_owned(),
        None => match refresh_auth_key(cookie_jar).await {
            Ok(result) => {
                cookie_jar = result.1;
                result.0
            }
            Err(err) => {
                cookie_jar = err.1;
                return (
                    StatusCode::UNAUTHORIZED,
                    cookie_jar,
                    Err(Json(new_error(StatusCode::UNAUTHORIZED, err.0).into())),
                );
            }
        },
    };

    let user_agent = match headers.get("User-Agent") {
        Some(header) => header.to_str().unwrap(),
        None => {
            return (StatusCode::UNAUTHORIZED, cookie_jar, Err(
                    Json(new_error(
                            StatusCode::UNAUTHORIZED,
                    "No User-Agent header found. This may be required for accessing the homeworker API".to_owned(),
                ).into()),
            ));
        }
    };

    let request_builder = match method {
        Method::GET => reqwest::Client::new().get("https://homeworker.li/".to_owned() + &path),
        Method::POST => reqwest::Client::new()
            .post("https://homeworker.li".to_owned() + &path)
            .body(body),
        Method::DELETE => reqwest::Client::new()
            .delete("https://homeworker.li".to_owned() + &path)
            .body(body),
        _ => todo!(),
    };

    let response = match request_builder
        .header("Authorization", "Bearer ".to_string() + &access_token)
        .header("User-Agent", user_agent)
        .header("Cookie", "auth-key=".to_owned() + &auth_key)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                cookie_jar,
                Err(Json(
                    new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Proxy could not reach the homeworker API".to_owned(),
                    )
                    .into(),
                )),
            );
        }
    };

    (
        response.status(),
        cookie_jar,
        Ok(response.text().await.unwrap()),
    )
}

async fn refresh_token(
    cookie_jar: PrivateCookieJar,
) -> Result<(String, PrivateCookieJar), (String, PrivateCookieJar)> {
    let refresh_token = match cookie_jar.get("refresh-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => {
            println!("Token refresh: No refresh token");
            return Err(("No access or refresh token found".to_owned(), cookie_jar));
        }
    };

    match homeworker::auth::refresh_token(
        std::env::var("CLIENT_ID").unwrap(),
        std::env::var("CLIENT_SECRET").unwrap(),
        refresh_token,
    )
    .await
    {
        Ok(response) => {
            // Set new access cookie
            println!("Token refresh: Success");
            Ok((
                response.access_token.clone(),
                cookie_jar.add(
                    Cookie::build("access-token", response.access_token.clone())
                        .http_only(true)
                        .secure(true)
                        .path("/homeworker")
                        .same_site(SameSite::Lax)
                        .max_age(Duration::seconds(response.expires_in as i64))
                        .finish(),
                ),
            ))
        }
        Err(e) => {
            println!("Token refresh: {:?}", e);
            return Err(("Unable to refresh the access token".to_owned(), cookie_jar));
        }
    }
}

/// Expects a valid access-token cookie
async fn refresh_auth_key(
    cookie_jar: PrivateCookieJar,
) -> Result<(String, PrivateCookieJar), (String, PrivateCookieJar)> {
    Ok(("PLACEHOLDER".to_owned(), cookie_jar))
    /*let access_token = match cookie_jar.get("access-token") {
        Some(cookie) => cookie.value().to_owned(),
        None => {
            println!("Auth-Key refresh: No access token");
            return Err(("No access token found".to_owned(), cookie_jar));
        }
    };

    match reqwest::Client::new()
        .get("https://homeworker.li/api/v2/me".to_owned())
        .header("Authorization", "Bearer ".to_owned() + &access_token)
        .send()
        .await
    {
        Ok(response) => {
            let auth_key = match response.headers().get("auth-key") {
                Some(h) => h.to_str().unwrap().to_owned(),
                None => {
                    println!("Auth-Key refresh: No auth-key cookie returned");
                    return Err((
                        "No auth-key cookie returned from homeworker".to_owned(),
                        cookie_jar,
                    ));
                }
            };
            Ok((
                auth_key.clone(),
                cookie_jar.add(
                    Cookie::build("auth-key", auth_key)
                        .http_only(true)
                        .secure(true)
                        .path("/homeworker")
                        .same_site(SameSite::Lax)
                        .finish(),
                ),
            ))
        }
        Err(e) => {
            println!("Auth-Key refresh: {:?}", e);
            return Err((
                "Unable to refresh the auth-key token".to_owned(),
                cookie_jar,
            ));
        }
    }*/
}

fn new_error(code: StatusCode, message: String) -> homeworker::types::Error {
    homeworker::types::Error {
        name: "Proxy".to_owned(),
        message,
        code: code.into(),
    }
}
