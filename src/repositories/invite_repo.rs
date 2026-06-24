use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
use crate::models::invite_model::{CreateResult, Invite};

pub struct InviteRepository;

impl InviteRepository {

    // CREATE INVITE
    pub async fn create(
        pool: &PgPool,
        outlet_id: Uuid,
        role: &str,
        token: &str,
    ) -> Result<CreateResult, sqlx::Error> {

        let token: String = sqlx::query_scalar(
            r#"
            INSERT INTO invites (id, outlet_id, role, token, expired_at)
            VALUES ($1, $2, $3, $4, NOW() + INTERVAL '1 day')
            RETURNING token
            "#
        )
        .bind(Uuid::new_v4())
        .bind(outlet_id)
        .bind(role)
        .bind(token)
        .fetch_one(pool)
        .await?;

        Ok(CreateResult { token })
    }

    // FIND BY TOKEN
    pub async fn find_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<Invite>, sqlx::Error> {

        let row = sqlx::query(
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
            "#
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Invite {
            id: r.get("id"),
            outlet_id: r.get("outlet_id"),
            role: r.get("role"),
            token: r.get("token"),
            expired_at: r.try_get("expired_at").ok(),
            used: r.try_get::<Option<bool>, _>("used").ok().flatten().unwrap_or(false),
            created_at: r.try_get("created_at").ok().unwrap(),
        }))
    }

    // MARK AS USED
    pub async fn used(
        pool: &PgPool,
        token: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Ambil data invite
        let invite = sqlx::query(
            r#"
            SELECT outlet_id, role
            FROM invites
            WHERE token = $1
            AND used = false
            "#
        )
        .bind(token)
        .fetch_one(&mut *tx)
        .await?;

        let outlet_id: Uuid = invite.get("outlet_id");
        let role: String = invite.get("role");

        // Generate user id
        let user_id = Uuid::new_v4();

        // Insert user
        sqlx::query(
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
            "#
        )
        .bind(user_id)
        .bind(name)
        .bind(email)
        .bind(password)
        .execute(&mut *tx)
        .await?;

        // Insert user_outlets
        sqlx::query(
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
            "#
        )
        .bind(user_id)
        .bind(outlet_id)
        .bind(role)
        .execute(&mut *tx)
        .await?;

        // Tandai invite sudah dipakai
        sqlx::query(
            r#"
            UPDATE invites
            SET used = true
            WHERE token = $1
            "#
        )
        .bind(token)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    // DELETE (optional cleanup)
    pub async fn delete(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {

        sqlx::query(
            r#"
            DELETE FROM invites WHERE id = $1
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
