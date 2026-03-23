use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub google_id: String,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize)]
pub struct UserResponse {
    id: Uuid,
    name: String,
    email: String,
    picture: Option<String>,
    created_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            picture: user.picture,
            created_at: user.created_at.to_string(),
        }
    }
}
