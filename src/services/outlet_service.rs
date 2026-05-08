use uuid::Uuid;
use sqlx::PgPool;

use crate::models::outlet_model::Outlet;
use crate::repositories::outlet_repo::OutletRepository;
use crate::dto::outlet_dto::UpdateOutletDTO;
use crate::utils::app_error::AppError;

pub struct OutletService;

impl OutletService {

    // 🔥 CREATE OUTLET
    pub async fn create_outlet(
        pool: &PgPool,
        user_id: Uuid,
        name: String,
        address_line: String,
        city: String,
        province: String,
        postal_code: Option<String>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Result<Uuid, AppError> {

        // validation
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("Outlet name is required".into()));
        }

        if address_line.trim().is_empty() {
            return Err(AppError::BadRequest("Address is required".into()));
        }

        if city.trim().is_empty() {
            return Err(AppError::BadRequest("City is required".into()));
        }

        if province.trim().is_empty() {
            return Err(AppError::BadRequest("Province is required".into()));
        }

        let outlet_id = OutletRepository::create(
            pool,
            &name,
            user_id,
            &address_line,
            &city,
            &province,
            postal_code.as_deref(),
            latitude,
            longitude,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(outlet_id)
    }

    // 🔍 GET ALL OUTLETS
    pub async fn get_all_outlets(
        pool: &PgPool,
    ) -> Result<Vec<Outlet>, AppError> {

        let outlets = OutletRepository::find_all(pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(outlets)
    }

    // 🔍 GET OUTLET BY ID
    pub async fn get_outlet_by_id(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Outlet, AppError> {

        let outlet = OutletRepository::find_by_id(
            pool,
            outlet_id,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?
        .ok_or_else(|| AppError::NotFound("Outlet not found".into()))?;

        Ok(outlet)
    }

    // 🔍 GET MY OUTLETS
    pub async fn get_my_outlets(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Outlet>, AppError> {

        let outlets = OutletRepository::find_by_owner(
            pool,
            user_id,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(outlets)
    }

    // ✏️ UPDATE OUTLET
    pub async fn update_outlet(
        pool: &PgPool,
        user_id: Uuid,
        outlet_id: Uuid,
        req: UpdateOutletDTO,
    ) -> Result<(), AppError> {

        // 🔍 cek dulu outlet ada atau tidak
        let outlet = OutletRepository::find_by_id(pool, outlet_id)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or_else(|| AppError::NotFound("Outlet not found".into()))?;

        // 🔐 authorization
        let is_owner = OutletRepository::is_owner(
            pool,
            outlet.id,
            user_id,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        if !is_owner {
            return Err(AppError::Unauthorized);
        }

        // validation
        if let Some(name) = &req.name {
            if name.trim().is_empty() {
                return Err(AppError::BadRequest("Outlet name cannot be empty".into()));
            }
        }

        if let Some(city) = &req.city {
            if city.trim().is_empty() {
                return Err(AppError::BadRequest("City cannot be empty".into()));
            }
        }

        if let Some(province) = &req.province {
            if province.trim().is_empty() {
                return Err(AppError::BadRequest("Province cannot be empty".into()));
            }
        }

        OutletRepository::update(
            pool,
            outlet_id,
            req,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }

    // ❌ DELETE OUTLET
    pub async fn delete_outlet(
        pool: &PgPool,
        user_id: Uuid,
        outlet_id: Uuid,
    ) -> Result<(), AppError> {

        // 🔍 cek dulu outlet ada atau tidak
        let outlet = OutletRepository::find_by_id(pool, outlet_id)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or_else(|| AppError::NotFound("Outlet not found".into()))?;

        // 🔐 authorization
        let is_owner = OutletRepository::is_owner(
            pool,
            outlet.id,
            user_id,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        if !is_owner {
            return Err(AppError::Unauthorized);
        }

        OutletRepository::delete(
            pool,
            outlet_id,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
}