use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_info: CpuInfo,
    pub memory_info: MemoryInfo,
    pub disk_info: Vec<DiskInfo>,
    pub network_info: NetworkInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub usage_percent: f32,
    pub temperature: Option<f32>,
    pub core_count: u32,
    pub frequency_mhz: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub status: NetworkInterfaceStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkInterfaceStatus {
    Up,
    Down,
    Unknown,
}
