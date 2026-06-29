use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::table_repo::TableRepository;
use crate::dto::table_dto::{CreateTableDTO, TableResponseDTO, UpdateTableDTO};

pub struct TableService;

impl TableService {

    pub async fn create(
        pool: &PgPool,
        outlet_id: Uuid,
        dto: CreateTableDTO,
    ) -> Result<TableResponseDTO, sqlx::Error> {

        let table = TableRepository::create(
            pool,
            outlet_id,
            dto.name,
            dto.location,
        ).await?;

        Ok(TableResponseDTO {
            id: table.id,
            outlet_id: table.outlet_id,
            name: table.name,
            location: table.location,
            status: table.status,
        })
    }

    pub async fn get_by_id(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<TableResponseDTO, sqlx::Error> {

        let table = TableRepository::get_by_id(pool, table_id).await?;

        Ok(TableResponseDTO {
            id: table.id,
            outlet_id: table.outlet_id,
            name: table.name,
            location: table.location,
            status: table.status,
        })
    }

    pub async fn get_by_outlet_id(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<TableResponseDTO>, sqlx::Error> {

        let tables = TableRepository::get_by_outlet_id(pool, outlet_id).await?;

        Ok(tables.into_iter().map(|table| TableResponseDTO {
            id: table.id,
            outlet_id: table.outlet_id,
            name: table.name,
            location: table.location,
            status: table.status,
        }).collect())
    }

    pub async fn update(
        pool: &PgPool,
        table_id: Uuid,
        dto: UpdateTableDTO,
    ) -> Result<TableResponseDTO, sqlx::Error> {

        // 🔥 validasi: minimal 1 field harus diisi
        if dto.name.is_none() && dto.location.is_none() && dto.status.is_none() {
            return Err(sqlx::Error::Protocol("No fields to update".into()));
        }

        let table = TableRepository::update(
            pool,
            table_id,
            dto.name,
            dto.location,
            dto.status,
        ).await?;

        Ok(TableResponseDTO {
            id: table.id,
            outlet_id: table.outlet_id,
            name: table.name,
            location: table.location,
            status: table.status,
        })
    }

    pub async fn delete_token(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<(), sqlx::Error> {

        TableRepository::delete_token(pool, table_id).await?;

        Ok(())
    }

    pub async fn delete(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<bool, sqlx::Error> {

        let result = TableRepository::delete(pool, table_id).await?;

        // 🔥 cek apakah ada row yang kehapus
        Ok(result.rows_affected() > 0)
    }

}