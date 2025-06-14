use super::provider::{DiskInfo, DiskProvider};

#[cfg(target_os = "windows")]
mod os_impl {
    pub use crate::disk::os::windows::*;
}

#[cfg(target_os = "linux")]
mod os_impl {
    pub use crate::disk::os::linux::*;
}

#[cfg(target_os = "macos")]
mod os_impl {
    pub use crate::disk::os::mac::*;
}

pub struct LocalDiskProvider;

impl DiskProvider for LocalDiskProvider {
    fn list_disks(&self) -> Vec<DiskInfo> {
        match os_impl::list_disks() {
            Ok(disks) => disks,
            Err(e) => {
                eprintln!("❌ خطا در شناسایی دیسک‌ها: {e}");
                vec![]
            }
        }
    }
}


