use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{
    db::Database,
    error::AppError,
    models::SystemInfo,
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/info/:device_id", get(get_system_info))
}

async fn get_system_info(
    State(db): State<Database>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<SystemInfo>, AppError> {
    // TODO: Implement system info retrieval
    Err(AppError::Internal("Not implemented".to_string()))
}
