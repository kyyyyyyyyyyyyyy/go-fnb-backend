use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;

use crate::models::invite_model::{CreateResult, Invite};
use crate::repositories::invite_repo::InviteRepository;
use crate::repositories::outlet_repo::OutletRepository;
use crate::utils::hash;

pub struct InviteService;

impl InviteService {

    // 🔥 CREATE INVITE
    pub async fn create_invite(
    pool: &PgPool,
    outlet_id: Uuid,
    role: &str,
    ) -> Result<CreateResult, String> {

        let outlet = OutletRepository::find_by_id(pool, outlet_id)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        // ✅ FIX 1: jangan pakai !
        if outlet.is_none() {
            return Err("Outlet not found".to_string());
        }

        let token = Uuid::new_v4().to_string();

        // pastikan ini return Invite, bukan CreateResult
        let invite = InviteRepository::create(
            pool,
            outlet_id,
            role,
            &token,
        )
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        // ✅ FIX 2: sekarang type sudah cocok
        Ok(invite)
    }

    // 🔍 VALIDATE INVITE BY TOKEN
    pub async fn validate_invite(
        pool: &PgPool,
        token: &str,
    ) -> Result<Invite, String> {

        let invite = InviteRepository::find_by_token(pool, token)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Invite not found")?;

        // ❌ cek apakah sudah dipakai
        if invite.used {
            return Err("Invite already used".into());
        }

        // ❌ cek expired
        if let Some(expired_at) = invite.expired_at {
            if expired_at < Utc::now().naive_utc() {
                return Err("Invite expired".into());
            }
        }

        Ok(invite)
    }

    // ✅ USE INVITE (mark as used)
    pub async fn use_invite(
        pool: &PgPool,
        token: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(), String> {

        // validasi dulu
        let _ = Self::validate_invite(pool, token).await?;

        let hashed_password = hash::hash_password(password);
        // mark as used
        InviteRepository::used(pool, token, name, email, &hashed_password)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    // ❌ DELETE INVITE (optional)
    pub async fn delete_invite(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), String> {

        InviteRepository::delete(pool, id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}