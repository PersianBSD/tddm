#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub name: Option<String>,
    pub volume_label: Option<String>,
    pub file_system: Option<String>,
    pub total_space: u64,
    pub free_space: u64,
    pub used_space: Option<u64>,
    pub is_removable: bool,
    pub mount_point: String,
}


pub use crate::partition::os::windows::list_partitions;


