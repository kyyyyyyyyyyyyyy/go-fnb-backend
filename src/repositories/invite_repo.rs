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
    pub async fn used(
        pool: &PgPool,
        token: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Ambil data invite
        let invite = sqlx::query!(
            r#"
            SELECT outlet_id, role
            FROM invites
            WHERE token = $1
            AND used = false
            "#,
            token
        )
        .fetch_one(&mut *tx)
        .await?;

        // Generate user id
        let user_id = Uuid::new_v4();

        // Hash password
        // Insert user
        sqlx::query!(
            r#"
            INSERT INTO users (
                id,
                name,
                email,
                password
            )
            VALUES (
                $1,
                $2,
                $3,
                $4
            )
            "#,
            user_id,
            name,
            email,
            password
        )
        .execute(&mut *tx)
        .await?;

        // Insert user_outlets
        sqlx::query!(
            r#"
            INSERT INTO user_outlets (
                user_id,
                outlet_id,
                role
            )
            VALUES (
                $1,
                $2,
                $3
            )
            "#,
            user_id,
            invite.outlet_id,
            invite.role
        )
        .execute(&mut *tx)
        .await?;

        // Tandai invite sudah dipakai
        sqlx::query!(
            r#"
            UPDATE invites
            SET used = true
            WHERE token = $1
            "#,
            token
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

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
