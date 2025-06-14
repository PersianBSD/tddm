use crate::disk::provider::DiskInfo;
use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ, FILE_SHARE_WRITE,
        OPEN_EXISTING,
    },
    Win32::System::Ioctl::{DISK_GEOMETRY_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX},
    Win32::System::IO::DeviceIoControl,
};

pub fn get_disk_size(handle: HANDLE) -> Option<u64> {
    let mut out_buffer = [0u8; std::mem::size_of::<DISK_GEOMETRY_EX>() + 1024];
    let mut returned = 0u32;

    let result = unsafe {
        DeviceIoControl(
            handle,
            IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
            None,
            0,
            Some(out_buffer.as_mut_ptr() as *mut _),
            out_buffer.len() as u32,
            Some(&mut returned),
            None,
        )
    };

    match result {
        Ok(_) => {
            let geometry = unsafe { &*(out_buffer.as_ptr() as *const DISK_GEOMETRY_EX) };
            Some(geometry.DiskSize.try_into().ok()?)
        }
        Err(_) => None,
    }
}

pub fn list_disks() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    let mut disks = vec![];

    for i in 0..=10 {
        let path = format!("\\\\.\\PhysicalDrive{}", i);
        let path_w: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

        let handle = unsafe {
            CreateFileW(
                PCWSTR(path_w.as_ptr()),
                FILE_GENERIC_READ.0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                HANDLE(0),
            )
        };

        match handle {
            Ok(h) if h.0 != INVALID_HANDLE_VALUE.0 => {
                println!("✅ {} باز شد.", path);

                let size = get_disk_size(h).unwrap_or(0);

                // دسته را ببند تا لیک نشود
               let _ = unsafe { CloseHandle(h) };
                disks.push(DiskInfo {
                    name: path,
                    size_gb: size / 1_000_000_000,
                    is_removable: false,
                });
            }
            _ => {
                // نادیده بگیر
            }
        }
    }

    Ok(disks)
}
