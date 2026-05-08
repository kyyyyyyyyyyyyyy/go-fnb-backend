use sqlx::PgPool;
use uuid::Uuid;
use crate::models::invite_model::{CreateResult, Invite};

pub struct InviteRepository;

impl InviteRepository {

    // 🔥 CREATE INVITE
    pub async fn create(
        pool: &PgPool,
        outlet_id: Uuid,
        role: &str,
        token: &str,
    ) -> Result<CreateResult, sqlx::Error> {

        let record = sqlx::query!(
            r#"
            INSERT INTO invites (id, outlet_id, role, token, expired_at)
            VALUES ($1, $2, $3, $4, NOW() + INTERVAL '1 day')
            RETURNING
            id,
            outlet_id,
            role,
            token,
            expired_at,
            used,
            created_at as "created_at!"
            "#,
            Uuid::new_v4(),
            outlet_id,
            role,
            token
        )
        .fetch_one(pool)
        .await?;

        Ok(CreateResult {
            token: record.token,
        })
    }

    // 🔍 FIND BY TOKEN
    pub async fn find_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<Invite>, sqlx::Error> {

        let record = sqlx::query!(
            r#"
            SELECT
                id,
                outlet_id,
                role,
                token,
                expired_at,
                used,
                created_at
            FROM invites
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(pool)
        .await?;

        Ok(record.map(|r| Invite {
            id: r.id,
            outlet_id: r.outlet_id,
            role: r.role,
            token: r.token,
            expired_at: r.expired_at,
            used: r.used.unwrap_or(false),
            created_at: r.created_at,
        }))
    }

    // ✅ MARK AS USED
    pub async fn mark_as_used(
        pool: &PgPool,
        token: &str,
    ) -> Result<(), sqlx::Error> {

        sqlx::query!(
            r#"
            UPDATE invites
            SET used = true
            WHERE token = $1
            "#,
            token
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // ❌ DELETE (optional cleanup)
    pub async fn delete(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {

        sqlx::query!(
            r#"
            DELETE FROM invites WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
