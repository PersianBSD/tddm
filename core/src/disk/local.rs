use super::provider::{DiskInfo, DiskProvider};
use sysinfo::{System, SystemExt, DiskExt};

pub struct LocalDiskProvider;

impl DiskProvider for LocalDiskProvider {
    fn list_disks(&self) -> Vec<DiskInfo> {
        let mut sys = System::new_all();
        sys.refresh_disks();

        sys.disks()
            .iter()
            .map(|disk| {
                DiskInfo {
                    name: disk.mount_point().to_string_lossy().to_string(),
                    size_gb: disk.total_space() / 1_073_741_824, // bytes → GB
                    is_removable: disk.is_removable(),
                }
            })
            .collect()
    }
}

