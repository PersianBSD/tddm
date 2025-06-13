// core/src/disk/provider.rs

#[derive(Debug)]
pub struct DiskInfo {
    pub name: String,
    pub size_gb: u64,
    pub is_removable: bool,
}

pub trait DiskProvider {
    fn list_disks(&self) -> Vec<DiskInfo>;
}
