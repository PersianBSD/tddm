// core/src/disk/provider.rs

#[derive(Debug)]
pub struct DiskInfo {
    pub name: String,
    pub size_gb: u64,
    pub is_removable: bool,
    pub model: Option<String>,     // ← اضافه کن
    pub serial: Option<String>,    // ← اضافه کن

}

#[derive(Debug, Clone)]
pub struct DiskExtraInfo {
    pub device_id: String,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub is_removable: bool,
}

pub trait DiskProvider {
    fn list_disks(&self) -> Vec<DiskInfo>;
}

