use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub os_type: String,
    pub os_version: String,
    pub status: DeviceStatus,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCreate {
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub os_type: String,
    pub os_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceUpdate {
    pub name: Option<String>,
    pub hostname: Option<String>,
    pub ip_address: Option<String>,
    pub status: Option<DeviceStatus>,
}
