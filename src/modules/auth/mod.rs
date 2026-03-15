pub mod error;
pub mod google;
pub mod jwt;

use axum::{
    Json,
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient, reqwest,
};
use serde::Deserialize;
use time::Duration;
use uuid::Uuid;

use crate::{
    AppState,
    modules::auth::{error::AuthError, google::fetch_userinfo, jwt::JwtPayload},
};

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

pub async fn google_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), String> {
    let client = BasicClient::new(ClientId::new(state.config.google_client_id.clone()))
        .set_client_secret(ClientSecret::new(state.config.google_client_secret.clone()))
        .set_auth_uri(AuthUrl::new(state.config.google_auth_uri.clone()).unwrap())
        .set_token_uri(TokenUrl::new(state.config.google_token_uri.clone()).unwrap())
        .set_redirect_uri(RedirectUrl::new(state.config.google_redirect_uri.clone()).unwrap());

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    let mut cookie = Cookie::new("oauth_state", csrf_token.secret().clone());
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_max_age(Duration::minutes(5));

    return Ok((jar.add(cookie), Redirect::to(auth_url.as_str())));
}

pub async fn callback_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<CallbackQuery>,
) -> Result<(CookieJar, Redirect), AuthError> {
    let oauth_state = jar
        .get("oauth_state")
        .map(|c| CsrfToken::new(c.value().to_owned()))
        .ok_or(AuthError::MissingCookie)?;

    if oauth_state.secret() != &query.state {
        AuthError::CsrfMismatch;
    }

    let client = BasicClient::new(ClientId::new(state.config.google_client_id.clone()))
        .set_client_secret(ClientSecret::new(state.config.google_client_secret.clone()))
        .set_auth_uri(AuthUrl::new(state.config.google_auth_uri.clone()).unwrap())
        .set_token_uri(TokenUrl::new(state.config.google_token_uri.clone()).unwrap())
        .set_redirect_uri(RedirectUrl::new(state.config.google_redirect_uri.clone()).unwrap());

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| AuthError::FailedHttpClient)?;

    println!("code: {}", &query.code);

    let token_response = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(&http_client)
        .await
        .map_err(|_| AuthError::FailedToExchangeToken)?;

    let access_token = token_response.access_token();
    let userinfo = fetch_userinfo(&http_client, access_token.secret()).await?;

    println!("{:?}", userinfo);

    // insert user in db and retrieve their id.
    let user_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (google_id, name, email, picture)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (google_id)
            DO UPDATE SET 
                name = EXCLUDED.name,
                picture = EXCLUDED.picture
            RETURNING id;",
    )
    .bind(&userinfo.id)
    .bind(&userinfo.name)
    .bind(&userinfo.email)
    .bind(&userinfo.picture)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| AuthError::FailedToUpsertUser)?;

    let current_time = time::OffsetDateTime::now_utc().unix_timestamp() as usize;

    let payload = jwt::JwtPayload {
        sub: user_id.to_string(),
        email: userinfo.email,
        iat: current_time,
        exp: current_time + (state.config.jwt_expiration * 60 * 60) as usize,
        jti: uuid::Uuid::new_v4().to_string(),
    };
    let token = jwt::encode_token(&payload, &state.config)?;

    // set the access token cookie.
    let mut cookie = Cookie::new("access_token", token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_max_age(Duration::hours(24));

    return Ok((
        jar.remove(Cookie::from("oauth_state")).add(cookie),
        Redirect::to("http://localhost:8080/auth/dashboard"),
    ));
}

pub async fn dashboard_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<JwtPayload>, AuthError> {
    let token = jar
        .get("access_token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::MissingCookie)?;

    let payload = jwt::decode_token(&token, &state.config)?;
    return Ok(Json(payload));
}
