use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{dto::product_dto::{CategoryItemDTO, ProductResponseDTO}};

pub struct ProductRepository;

#[derive(sqlx::FromRow)]
struct ProductRow {
    product_id: Uuid,
    product_name: String,
    capital_price: i64,
    tax: i64,
    profit: i64,
    price: i64,
    image_url: String,
    available: bool,

    category_id: Option<Uuid>,
    category_name: Option<String>,
}

impl ProductRepository {


    pub async fn create_product(
        pool: &PgPool,
        name: String,
        capital_price: i64,
        tax: i64,
        profit: i64,
        price: i64,
        image_url: String,
        category_ids: Vec<Uuid>,
        outlet_id: Uuid,
    ) -> Result<(), sqlx::Error> {

        let mut tx =
            pool.begin().await?;

        let product_id: Uuid =
            sqlx::query_scalar(
                r#"
                INSERT INTO products (
                    name,
                    capital_price,
                    tax,
                    profit,
                    price,
                    image_url,
                    outlet_id
                )
                VALUES (
                    $1,$2,$3,$4,$5,$6,$7
                )
                RETURNING id
                "#
            )
            .bind(name)
            .bind(capital_price)
            .bind(tax)
            .bind(profit)
            .bind(price)
            .bind(image_url)
            .bind(outlet_id)
            .fetch_one(&mut *tx)
            .await?;

            for category_id in category_ids {

                sqlx::query(
                    r#"
                    INSERT INTO product_categories
                    (
                        product_id,
                        category_id
                    )
                    VALUES
                    (
                        $1,
                        $2
                    )
                    "#
                )
                .bind(product_id)
                .bind(category_id)
                .execute(&mut *tx)
                .await?;
            }

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_product_by_outlet(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<ProductResponseDTO>, sqlx::Error> {

        let rows =
        sqlx::query_as::<_, ProductRow>(
            r#"
            SELECT
                p.id AS product_id,
                p.name AS product_name,

                p.capital_price,
                p.tax,
                p.profit,
                p.price,
                p.image_url,
                p.available,

                c.id AS category_id,
                c.name AS category_name

            FROM products p

            LEFT JOIN product_categories pc
                ON pc.product_id = p.id

            LEFT JOIN categories c
                ON c.id = pc.category_id

            WHERE p.outlet_id = $1
            "#
        )
        .bind(outlet_id)
        .fetch_all(pool)
        .await?;

        let mut map =
            std::collections::HashMap::<Uuid, ProductResponseDTO>::new();

        for row in rows {

            let product =
                map.entry(row.product_id)
                    .or_insert(
                        ProductResponseDTO {
                            id: row.product_id,
                            name: row.product_name,
                            capital_price: row.capital_price,
                            tax: row.tax,
                            profit: row.profit,
                            price: row.price,
                            image_url: row.image_url,
                            available: row.available,
                            categories: vec![],
                        }
                    );

            if let Some(category_id) = row.category_id {

                product.categories.push(
                    CategoryItemDTO {
                        id: category_id,
                        name: row.category_name.unwrap(),
                    }
                );
            }
        }

        Ok(map.into_values().collect())
    }
    
    pub async fn get_product_by_id(
        pool: &PgPool,
        product_id: Uuid,
    ) -> Result<Option<ProductResponseDTO>, sqlx::Error> {

        let rows =
            sqlx::query_as::<_, ProductRow>(
                r#"
                SELECT
                    p.id AS product_id,
                    p.name AS product_name,

                    p.capital_price,
                    p.tax,
                    p.profit,
                    p.price,
                    p.image_url,
                    p.available,

                    c.id AS category_id,
                    c.name AS category_name

                FROM products p

                LEFT JOIN product_categories pc
                    ON pc.product_id = p.id

                LEFT JOIN categories c
                    ON c.id = pc.category_id

                WHERE p.id = $1
                "#
            )
            .bind(product_id)
            .fetch_all(pool)
            .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let mut map =
            std::collections::HashMap::<Uuid, ProductResponseDTO>::new();

        for row in rows {
            let product =
                map.entry(row.product_id)
                    .or_insert(
                        ProductResponseDTO {
                            id: row.product_id,
                            name: row.product_name,
                            capital_price: row.capital_price,
                            tax: row.tax,
                            profit: row.profit,
                            price: row.price,
                            image_url: row.image_url,
                            available: row.available,
                            categories: vec![],
                        }
                    );

            if let Some(category_id) = row.category_id {
                product.categories.push(
                    CategoryItemDTO {
                        id: category_id,
                        name: row.category_name.unwrap(),
                    }
                );
            }
        }

        Ok(map.into_values().next())
    }

    pub async fn update_product(
        pool: &PgPool,
        product_id: Uuid,

        name: Option<String>,
        capital_price: Option<i64>,
        tax: Option<i64>,
        profit: Option<i64>,
        price: Option<i64>,
        image_url: Option<String>,

        add_category_ids: Option<Vec<Uuid>>,
        remove_category_ids: Option<Vec<Uuid>>,
    ) -> Result<bool, sqlx::Error> {

        let mut tx: Transaction<'_, Postgres> =
            pool.begin().await?;

        let result =
            sqlx::query(
                r#"
                UPDATE products
                SET
                    name = COALESCE($1, name),
                    capital_price = COALESCE($2, capital_price),
                    tax = COALESCE($3, tax),
                    profit = COALESCE($4, profit),
                    price = COALESCE($5, price),
                    image_url = COALESCE($6, image_url)

                WHERE id = $7
                "#
            )
            .bind(name)
            .bind(capital_price)
            .bind(tax)
            .bind(profit)
            .bind(price)
            .bind(image_url)
            .bind(product_id)
            .execute(&mut *tx)
            .await?;

        // tidak ada product yang terupdate
        if result.rows_affected() == 0 {
            tx.rollback().await?;
            return Ok(false);
        }

        if let Some(add_ids) = add_category_ids {

            for category_id in add_ids {

                sqlx::query(
                    r#"
                    INSERT INTO product_categories
                    (
                        product_id,
                        category_id
                    )
                    VALUES
                    (
                        $1,
                        $2
                    )
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(product_id)
                .bind(category_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        if let Some(remove_ids) = remove_category_ids {

            sqlx::query(
                r#"
                DELETE FROM product_categories
                WHERE product_id = $1
                AND category_id = ANY($2)
                "#
            )
            .bind(product_id)
            .bind(&remove_ids)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(true)
    }

    pub async fn update_product_available(
        pool: &PgPool,
        product_id: Uuid,
        available: bool,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE products
            SET available = $1
            WHERE id = $2
            "#
        )
        .bind(available)
        .bind(product_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_product(
        pool: &PgPool,
        product_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM products
            WHERE id = $1
            "#
        )
        .bind(product_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }


}