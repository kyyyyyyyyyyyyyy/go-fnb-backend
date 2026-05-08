use sqlx::PgPool;
use std::env;

use crate::{
    repositories::user_repo,
    utils::{hash, jwt},
    models::oauth_model::{GoogleTokenResponse, GoogleUserInfo},
};

pub async fn register(
    db: &PgPool,
    name: String,
    email: String,
    password: String,
) -> Result<(), String> {
    let hashed = hash::hash_password(&password);

    user_repo::create_user(db, &name, &email, &hashed)
        .await
        .map_err(|_| "Failed to create user".to_string())
}

pub async fn login(
    db: &PgPool,
    email: String,
    password: String,
) -> Result<String, String> {

    let email = email.to_lowercase();

    let user = user_repo::find_by_email(db, &email)
        .await
        .map_err(|_| "DB error".to_string())?
        .ok_or("Invalid credentials")?;

    // 🔥 blok login jika user google
    if user.provider.as_deref() == Some("google") {
        return Err("Use Google login".to_string());
    }

    let hashed = user.password.ok_or("Invalid credentials")?;

    if !hash::verify_password(&password, &hashed) {
        return Err("Invalid credentials".to_string());
    }

    Ok(jwt::generate_token(&user.id))
}

pub async fn exchange_code(code: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let client_id = env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| "Missing GOOGLE_CLIENT_ID")?;

    let client_secret = env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| "Missing GOOGLE_CLIENT_SECRET")?;

    let redirect_uri = env::var("GOOGLE_REDIRECT_URI")
        .map_err(|_| "Missing GOOGLE_REDIRECT_URI")?;

    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_uri.as_str()),
    ];

    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    if !res.status().is_success() {
        return Err("Failed exchange code (Google rejected)".to_string());
    }

    let body: GoogleTokenResponse = res
        .json()
        .await
        .map_err(|_| "Invalid token response")?;

    Ok(body.access_token)
}

pub async fn get_user_info(token: &str) -> Result<GoogleUserInfo, String> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    if !res.status().is_success() {
        return Err("Failed get user info".to_string());
    }

    let user = res
        .json::<GoogleUserInfo>()
        .await
        .map_err(|_| "Invalid user info")?;

    Ok(user)
}

pub async fn handle_google_callback(
    db: &PgPool,
    code: &str,
) -> Result<String, String> {

    let token = exchange_code(code).await?;
    let user_info = get_user_info(&token).await?;

    let user = match user_repo::find_by_provider_id(db, &user_info.id)
        .await
        .map_err(|_| "DB error")?
    {
        Some(user) => user,

        None => {
            user_repo::create_user_google(
                db,
                &user_info.name,
                &user_info.email,
                &user_info.id,
            )
            .await
            .map_err(|_| "Failed create user")?
        }
    };

    Ok(jwt::generate_token(&user.id))
}
