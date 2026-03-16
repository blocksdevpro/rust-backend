pub mod error;
pub mod google;
pub mod jwt;

use axum::{
    Json, Router,
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use serde::Deserialize;
use time::Duration;
use uuid::Uuid;

use crate::{
    AppState,
    modules::{
        auth::error::AuthError,
        users::model::{User, UserResponse},
    },
};

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

async fn google_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    // generate the authorize url.
    let (auth_url, csrf_token) = google::authorize_url(&state.config);

    // store the csrf token in a cookie.
    let mut cookie = Cookie::new("oauth_state", csrf_token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_max_age(Duration::minutes(5));

    // redirect to the authorize url.
    Ok((jar.add(cookie), Redirect::to(&auth_url)))
}

async fn callback_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<CallbackQuery>,
) -> Result<(CookieJar, Redirect), AuthError> {
    // get the csrf token from the cookie.

    let oauth_state = jar
        .get("oauth_state")
        .ok_or(AuthError::MissingCookie)?
        .value()
        .to_string();

    // verify the csrf token.
    if oauth_state != query.state {
        return Err(AuthError::CsrfMismatch);
    }

    // exchange the code for a token.
    let token_response =
        google::exchange_code(&state.http_client, &query.code, &state.config).await?;

    let access_token = token_response.access_token;
    let userinfo = google::fetch_userinfo(&state.http_client, &access_token).await?;

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
    .map_err(|e| {
        tracing::error!("Failed to upsert user, Err: {}", e);
        AuthError::FailedToUpsertUser
    })?;

    let current_time = time::OffsetDateTime::now_utc().unix_timestamp() as usize;

    let payload = jwt::JwtPayload {
        sub: user_id,
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

    Ok((
        jar.remove(Cookie::from("oauth_state")).add(cookie),
        Redirect::to("http://localhost:8080/auth/dashboard"),
    ))
}

async fn get_self_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<UserResponse>, AuthError> {
    let token = jar
        .get("access_token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::MissingCookie)?;

    let payload = jwt::decode_token(&token, &state.config)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(payload.sub)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch user, Err: {}", e);
            AuthError::InvalidToken
        })?;

    Ok(Json(UserResponse::from(user)))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/auth/google", get(google_handler))
        .route("/auth/callback", get(callback_handler))
        .route("/auth/me", get(get_self_handler))
}
