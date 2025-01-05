use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    db::Database,
    error::AppError,
    models::{Device, DeviceCreate, DeviceUpdate},
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_devices))
        .route("/", post(create_device))
        .route("/:id", get(get_device))
        .route("/:id", put(update_device))
        .route("/:id", delete(delete_device))
}

async fn list_devices(State(db): State<Database>) -> Result<Json<Vec<Device>>, AppError> {
    // TODO: Implement device listing
    Ok(Json(vec![]))
}

async fn create_device(
    State(db): State<Database>,
    Json(device): Json<DeviceCreate>,
) -> Result<Json<Device>, AppError> {
    // TODO: Implement device creation
    Err(AppError::Internal("Not implemented".to_string()))
}

async fn get_device(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Device>, AppError> {
    // TODO: Implement device retrieval
    Err(AppError::Internal("Not implemented".to_string()))
}

async fn update_device(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(update): Json<DeviceUpdate>,
) -> Result<Json<Device>, AppError> {
    // TODO: Implement device update
    Err(AppError::Internal("Not implemented".to_string()))
}

async fn delete_device(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    // TODO: Implement device deletion
    Err(AppError::Internal("Not implemented".to_string()))
}
