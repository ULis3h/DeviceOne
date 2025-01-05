use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    db::Database,
    error::AppError,
    models::{User, UserCreate, UserLogin, UserUpdate},
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .route("/:id", put(update_user))
        .route("/login", post(login))
}

async fn list_users(State(db): State<Database>) -> Result<Json<Vec<User>>, AppError> {
    // TODO: Implement user listing
    Ok(Json(vec![]))
}

async fn create_user(
    State(db): State<Database>,
    Json(user): Json<UserCreate>,
) -> Result<Json<User>, AppError> {
    // TODO: Implement user creation
    Err(AppError::Internal("Not implemented".to_string()))
}

async fn get_user(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    // TODO: Implement user retrieval
    Err(AppError::Internal("Not implemented".to_string()))
}

async fn update_user(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(update): Json<UserUpdate>,
) -> Result<Json<User>, AppError> {
    // TODO: Implement user update
    Err(AppError::Internal("Not implemented".to_string()))
}

async fn login(
    State(db): State<Database>,
    Json(credentials): Json<UserLogin>,
) -> Result<Json<String>, AppError> {
    // TODO: Implement login logic and return JWT token
    Err(AppError::Internal("Not implemented".to_string()))
}
