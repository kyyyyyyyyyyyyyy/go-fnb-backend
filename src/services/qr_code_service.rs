use uuid::Uuid;
use sqlx::PgPool;
use rand::{distributions::Alphanumeric, Rng};

use crate::repositories::qr_code_repo::QrCodeRepository;
use crate::dto::qr_code_dto::{CreateQrDTO, CreateQrWithTablesDTO, QrResponseDTO};
use crate::dto::qr_scan_dto::{AvailableTableDTO, ScanQrResponseDTO, TableDTO};
use crate::repositories::table_repo::TableRepository;

pub struct QrCodeService;

impl QrCodeService {

    // 🔥 CREATE QR + attach tables me
    pub async fn create(
        pool: &PgPool,
        dto: &CreateQrDTO,
    ) -> Result<QrResponseDTO, sqlx::Error> {

        // generate slug random
        let slug: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        // insert QR
        let qr = QrCodeRepository::create_qr(
            pool,
            dto.outlet_id,
            slug.clone(),
        ).await?;

        let table_ids = dto.table_ids.clone();

        // attach tables (pivot)
        QrCodeRepository::attach_tables(
            pool,
            qr.id,
            table_ids.clone() // ⚠️ clone karena Vec
        ).await?;

        Ok(QrResponseDTO {
            id: qr.id,
            slug,
            table_ids, // ⚠️ clone lagi
        })
    }

    // 🔍 SCAN QR (core logic kamu nanti di sini)
    // pub async fn scan(
    //     pool: &PgPool,
    //     slug: &str,
    // ) -> Result<Option<ScanQrResponseDTO>, sqlx::Error> {

    //     // cari QR berdasarkan slug
    //     let qr = match QrCodeRepository::find_by_slug(pool, slug).await? {
    //         Some(q) => q,
    //         None => return Ok(None),
    //     };

    //     // ambil tables dari pivot
    //     let tables = QrCodeRepository::get_tables_by_qr(pool, qr.id).await?;

    //     // mapping ke DTO
    //     let table_dtos: Vec<TableDTO> = tables.into_iter().map(|t| TableDTO {
    //         id: t.id,
    //         name: t.name,
    //     }).collect();

    //     Ok(Some(ScanQrResponseDTO {
    //         qr_id: qr.id,
    //         tables: table_dtos,
    //     }))
    // }


    pub async fn scan(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<ScanQrResponseDTO>, sqlx::Error> {

        let qr = match QrCodeRepository::find_by_slug(pool, slug).await? {
            Some(q) => q,
            None => return Ok(None),
        };

        let tables =
            TableRepository::get_available_tables_by_qr(pool, qr.id).await?;

        // tidak ada meja tersedia
        if tables.is_empty() {
            return Ok(None);
        }

        // =========================================
        // SINGLE TABLE
        // =========================================
        if tables.len() == 1 {

            let table = &tables[0];

            let token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();

            TableRepository::set_table_token(
                pool,
                table.id,
                &token,
            )
            .await?;

            return Ok(Some(ScanQrResponseDTO {
                qr_id: qr.id,
                qr_type: "single".to_string(),

                outlet_id: qr.outlet_id,

                table: Some(AvailableTableDTO {
                    id: table.id,
                    name: table.name.clone(),
                }),

                tables: None,
            }));
        }

        // =========================================
        // MULTI TABLE
        // =========================================

        let table_dtos = tables
            .into_iter()
            .map(|t| AvailableTableDTO {
                id: t.id,
                name: t.name,
            })
            .collect();

        Ok(Some(ScanQrResponseDTO {
            qr_id: qr.id,
            qr_type: "multi".to_string(),

            outlet_id: qr.outlet_id,

            table: None,

            tables: Some(table_dtos),
        }))
    }

    pub async fn select_table(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<String, sqlx::Error> {

        let table =
            TableRepository::find_by_id(pool, table_id).await?;

        let table = match table {
            Some(t) => t,
            None => {
                return Err(sqlx::Error::RowNotFound);
            }
        };

        // cek status meja
        if table.status != "available" {
            return Err(sqlx::Error::RowNotFound);
        }

        // generate token
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        // update token + status
        TableRepository::set_table_token(
            pool,
            table_id,
            &token,
        )
        .await?;

        Ok(token)
    }

    pub async fn create_with_tables(
        pool: &PgPool,
        dto: CreateQrWithTablesDTO,
    ) -> Result<QrResponseDTO, sqlx::Error> {

        // 🔥 1. generate slug
        let slug: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        // 🔥 2. create tables dulu
        let mut table_ids = Vec::new();

        for table_item in dto.tables {
            let table = TableRepository::create(
                pool,
                dto.outlet_id,
                table_item.name,
                Some(table_item.location),
            ).await?;

            table_ids.push(table.id);
        }

        // 🔥 3. create QR
        let qr = QrCodeRepository::create_qr(
            pool,
            dto.outlet_id,
            slug.clone(),
        ).await?;

        // 🔥 4. attach ke pivot
        QrCodeRepository::attach_tables(
            pool,
            qr.id,
            table_ids.clone(),
        ).await?;

        Ok(QrResponseDTO {
            id: qr.id,
            slug,
            table_ids,
        })
    }

    // 🔥 GET TABLES BY QR ID (optional utility)
    pub async fn get_tables(
        pool: &PgPool,
        qr_id: Uuid,
    ) -> Result<Vec<TableDTO>, sqlx::Error> {

        let tables = QrCodeRepository::get_tables_by_qr(pool, qr_id).await?;

        Ok(tables.into_iter().map(|t| TableDTO {
            id: t.id,
            name: t.name,
        }).collect())
    }


    pub async fn get_by_outlet(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<QrResponseDTO>, sqlx::Error> {

        let qrs = QrCodeRepository::get_by_outlet_id(pool, outlet_id).await?;

        let mut results = Vec::new();

        for qr in qrs {
            let tables = QrCodeRepository::get_tables_by_qr(pool, qr.id).await?;

            results.push(QrResponseDTO {
                id: qr.id,
                slug: qr.slug,
                table_ids: tables.into_iter().map(|t| t.id).collect(),
            });
        }

        Ok(results)
    }


    pub async fn regenerate_slug(
        pool: &PgPool,
        qr_id: Uuid,
    ) -> Result<QrResponseDTO, sqlx::Error> {

        // 🔥 retry kalau slug duplicate
        let mut attempts = 0;

        loop {
            if attempts > 5 {
                return Err(sqlx::Error::Protocol("Failed to generate unique slug".into()));
            }

            let new_slug: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

            let result = QrCodeRepository::regenerate_slug(
                pool,
                qr_id,
                new_slug.clone(),
            ).await;

            match result {
                Ok(qr) => {
                    return Ok(QrResponseDTO {
                        id: qr.id,
                        slug: qr.slug,
                        table_ids: vec![], // 🔥 optional (bisa diisi kalau mau join)
                    });
                }
                Err(err) => {
                    // 🔥 cek unique violation → retry
                    if let sqlx::Error::Database(db_err) = &err {
                        if db_err.constraint() == Some("qr_codes_slug_key") {
                            attempts += 1;
                            continue;
                        }
                    }

                    return Err(err);
                }
            }
        }
    }

}