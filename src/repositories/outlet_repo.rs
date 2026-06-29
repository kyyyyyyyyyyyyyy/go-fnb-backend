use sqlx::{PgPool, Row, QueryBuilder};
use uuid::Uuid;

use crate::models::outlet_model::Outlet;
use crate::dto::outlet_dto::{UpdateOutletDTO, OutletWithTodayStats};

pub struct OutletRepository;

impl OutletRepository {

    // CREATE OUTLET
    pub async fn create(
        pool: &PgPool,
        name: &str,
        owner_id: Uuid,
        address_line: &str,
        city: &str,
        province: &str,
        postal_code: Option<&str>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Result<Uuid, sqlx::Error> {

        let mut tx = pool.begin().await?;

        // insert address
        let address_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO addresses (
                address_line,
                city,
                province,
                postal_code,
                latitude,
                longitude
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(address_line)
        .bind(city)
        .bind(province)
        .bind(postal_code)
        .bind(latitude)
        .bind(longitude)
        .fetch_one(&mut *tx)
        .await?;

        // insert outlet
        let outlet_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO outlets (
                name,
                owner_id,
                address_id
            )
            VALUES ($1, $2, $3)
            RETURNING id
            "#
        )
        .bind(name)
        .bind(owner_id)
        .bind(address_id)
        .fetch_one(&mut *tx)
        .await?;

        // sync owner to user_outlets
        sqlx::query(
            r#"
            INSERT INTO user_outlets (
                user_id,
                outlet_id,
                role
            )
            VALUES ($1, $2, 'owner')
            "#
        )
        .bind(owner_id)
        .bind(outlet_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(outlet_id)
    }

    // GET ALL OUTLETS
    pub async fn find_all(
        pool: &PgPool,
    ) -> Result<Vec<Outlet>, sqlx::Error> {

        let rows = sqlx::query(
            r#"
            SELECT
                o.id,
                o.name,
                o.owner_id,
                a.address_line,
                a.city,
                a.province,
                a.postal_code,
                a.latitude,
                a.longitude
            FROM outlets o
            JOIN addresses a
                ON a.id = o.address_id
            ORDER BY o.created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_outlet).collect())
    }

    // FIND BY ID
    pub async fn find_by_id(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Option<Outlet>, sqlx::Error> {

        let row = sqlx::query(
            r#"
            SELECT
                o.id,
                o.name,
                o.owner_id,
                a.address_line,
                a.city,
                a.province,
                a.postal_code,
                a.latitude,
                a.longitude
            FROM outlets o
            JOIN addresses a
                ON a.id = o.address_id
            WHERE o.id = $1
            "#
        )
        .bind(outlet_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Self::map_outlet))
    }

    // FIND BY OWNER
    pub async fn find_by_owner(
        pool: &PgPool,
        owner_id: Uuid,
    ) -> Result<Vec<Outlet>, sqlx::Error> {

        let rows = sqlx::query(
            r#"
            SELECT
                o.id,
                o.name,
                o.owner_id,
                a.address_line,
                a.city,
                a.province,
                a.postal_code,
                a.latitude,
                a.longitude
            FROM outlets o
            JOIN addresses a
                ON a.id = o.address_id
            WHERE o.owner_id = $1
            ORDER BY o.created_at DESC
            "#
        )
        .bind(owner_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_outlet).collect())
    }

    // FIND BY OWNER WITH TODAY STATS
    pub async fn find_by_owner_with_today_stats(
        pool: &PgPool,
        owner_id: Uuid,
    ) -> Result<Vec<OutletWithTodayStats>, sqlx::Error> {

        let rows = sqlx::query(
            r#"
            SELECT
                o.id, o.name, o.owner_id,
                a.address_line, a.city, a.province,
                a.postal_code, a.latitude, a.longitude,
                COALESCE(SUM(ord.total) FILTER (WHERE ord.created_at >= CURRENT_DATE), 0)::BIGINT AS today_revenue,
                COALESCE(COUNT(ord.id) FILTER (WHERE ord.created_at >= CURRENT_DATE), 0)::BIGINT AS today_orders
            FROM outlets o
            JOIN addresses a ON a.id = o.address_id
            LEFT JOIN orders ord ON ord.outlet_id = o.id
            WHERE o.owner_id = $1
            GROUP BY o.id, o.name, o.owner_id, a.address_line, a.city, a.province,
                     a.postal_code, a.latitude, a.longitude
            ORDER BY o.created_at DESC
            "#
        )
        .bind(owner_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_outlet_with_stats).collect())
    }

    // UPDATE OUTLET
    pub async fn update(
        pool: &PgPool,
        outlet_id: Uuid,
        req: UpdateOutletDTO,
    ) -> Result<(), sqlx::Error> {

        let mut tx = pool.begin().await?;

        // update outlet name
        if let Some(name) = &req.name {
            sqlx::query(
                r#"
                UPDATE outlets
                SET name = $1
                WHERE id = $2
                "#
            )
            .bind(name)
            .bind(outlet_id)
            .execute(&mut *tx)
            .await?;
        }

        // get address_id
        let address_id: Uuid = sqlx::query_scalar(
            r#"
            SELECT address_id
            FROM outlets
            WHERE id = $1
            "#
        )
        .bind(outlet_id)
        .fetch_one(&mut *tx)
        .await?;

        // dynamic update address
        let mut qb = QueryBuilder::new("UPDATE addresses SET ");

        let mut has_update = false;

        if let Some(val) = &req.address_line {
            qb.push("address_line = ").push_bind(val);
            has_update = true;
        }

        if let Some(val) = &req.city {
            if has_update { qb.push(", "); }
            qb.push("city = ").push_bind(val);
            has_update = true;
        }

        if let Some(val) = &req.province {
            if has_update { qb.push(", "); }
            qb.push("province = ").push_bind(val);
            has_update = true;
        }

        if let Some(val) = &req.postal_code {
            if has_update { qb.push(", "); }
            qb.push("postal_code = ").push_bind(val);
            has_update = true;
        }

        if let Some(val) = req.latitude {
            if has_update { qb.push(", "); }
            qb.push("latitude = ").push_bind(val);
            has_update = true;
        }

        if let Some(val) = req.longitude {
            if has_update { qb.push(", "); }
            qb.push("longitude = ").push_bind(val);
            has_update = true;
        }

        if has_update {
            qb.push(" WHERE id = ").push_bind(address_id);

            qb.build()
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    // DELETE OUTLET
    pub async fn delete(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<(), sqlx::Error> {

        let mut tx = pool.begin().await?;

        // get address_id
        let address_id: Option<Uuid> = sqlx::query_scalar(
            r#"
            SELECT address_id
            FROM outlets
            WHERE id = $1
            "#
        )
        .bind(outlet_id)
        .fetch_optional(&mut *tx)
        .await?;

        // delete relations
        sqlx::query(
            r#"
            DELETE FROM user_outlets
            WHERE outlet_id = $1
            "#
        )
        .bind(outlet_id)
        .execute(&mut *tx)
        .await?;

        // delete outlet
        sqlx::query(
            r#"
            DELETE FROM outlets
            WHERE id = $1
            "#
        )
        .bind(outlet_id)
        .execute(&mut *tx)
        .await?;

        // delete address
        if let Some(address_id) = address_id {
            sqlx::query(
                r#"
                DELETE FROM addresses
                WHERE id = $1
                "#
            )
            .bind(address_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    // CHECK OWNER
    pub async fn is_owner(
        pool: &PgPool,
        outlet_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {

        let exists: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_outlets
                WHERE outlet_id = $1
                AND user_id = $2
                AND role = 'owner'
            )
            "#
        )
        .bind(outlet_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }

    pub async fn query_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<bool, sqlx::Error> {

        let result: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM outlets WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    fn map_outlet_with_stats(r: sqlx::postgres::PgRow) -> OutletWithTodayStats {

        OutletWithTodayStats {
            id: r.get("id"),
            name: r.get("name"),
            owner_id: r.get("owner_id"),
            address_line: r.get("address_line"),
            city: r.get("city"),
            province: r.get("province"),
            postal_code: r.try_get("postal_code").ok(),
            latitude: r.try_get("latitude").ok(),
            longitude: r.try_get("longitude").ok(),
            today_revenue: r.get("today_revenue"),
            today_orders: r.get("today_orders"),
        }
    }

    // MAPPER
    fn map_outlet(r: sqlx::postgres::PgRow) -> Outlet {

        Outlet {
            id: r.get("id"),
            name: r.get("name"),
            owner_id: r.get("owner_id"),
            address_line: r.get("address_line"),
            city: r.get("city"),
            province: r.get("province"),
            postal_code: r.try_get("postal_code").ok(),
            latitude: r.try_get("latitude").ok(),
            longitude: r.try_get("longitude").ok(),
        }
    }
}
