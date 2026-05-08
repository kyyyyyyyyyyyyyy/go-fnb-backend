use sqlx::PgPool;
use crate::models::user_model::User;

pub async fn create_user(
    db: &PgPool,
    name: &str,
    email: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        name,
        email,
        password
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn create_admin_outlet(
    db: &PgPool,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO outlets (id, name, owner_id) VALUES ($1, 'Admin Outlet', $2)",
        uuid::Uuid::new_v4(),
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn create_user_google(
    db: &PgPool,
    name: &str,
    email: &str,
    provider_id: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, provider, provider_id)
         VALUES ($1, $2, 'google', $3)
         RETURNING id, name, email, password, provider, provider_id, created_at",
        name,
        email,
        provider_id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn find_by_email(
    db: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password, provider, provider_id, created_at
         FROM users WHERE email = $1",
        email
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn find_by_provider_id(
    db: &PgPool,
    provider_id: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password, provider, provider_id, created_at
         FROM users WHERE provider_id = $1",
        provider_id
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}
