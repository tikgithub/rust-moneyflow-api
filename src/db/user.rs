use sqlx::query;

pub async fn has_with_email(db: &sqlx::PgPool, email: &str) -> bool{
    query!("SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await
        .unwrap()
        .is_some()
}

pub async fn create(db: &sqlx::PgPool, email: &str, password: &str) -> bool{
    query!("INSERT INTO users (email, password) VALUES ($1,$2) ", email, password)
        .execute(db)
        .await
        .is_ok()
}