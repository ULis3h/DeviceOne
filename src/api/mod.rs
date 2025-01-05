use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::db::Database;

mod device;
mod system;
mod user;

pub fn create_router(db: Database) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api/devices", device::router())
        .nest("/api/system", system::router())
        .nest("/api/users", user::router())
        .layer(cors)
        .with_state(db)
}
