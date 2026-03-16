use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub google_id: String,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
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
            created_at: user.created_at.format(&Rfc3339).unwrap(),
        }
    }
}
