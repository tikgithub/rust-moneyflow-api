use crate::controller::auth::SignUpRequest;
use bcrypt::{DEFAULT_COST, hash};
use sqlx::types::{BigDecimal, chrono};
use sqlx::{FromRow, query, query_as};

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub balance: Option<BigDecimal>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn has_with_email(db: &sqlx::PgPool, email: &str) -> bool {
    query!("SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await
        .unwrap()
        .is_some()
}

pub async fn create(db: &sqlx::PgPool, user: &SignUpRequest) -> bool {
    let hash_password = hash(&user.password, DEFAULT_COST).unwrap();

    query!(
        "INSERT INTO users (email, password, firstname, lastname) VALUES ($1,$2, $3, $4) ",
        &user.email,
        hash_password,
        &user.firstname,
        &user.lastname
    )
    .execute(db)
    .await
    .is_ok()
}

pub async fn get_by_email(db: &sqlx::PgPool, email: &str) -> Option<User> {
    query_as!(User,"SELECT id,email,password, firstname, lastname, balance, created_at, updated_at FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await
        .unwrap()
}
