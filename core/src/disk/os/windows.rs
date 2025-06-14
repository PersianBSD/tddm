use serde::Deserialize;
use std::collections::HashMap;
use crate::disk::provider::DiskExtraInfo;
use crate::disk::provider::DiskInfo;
use wmi::{COMLibrary, Variant, WMIConnection};
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32DiskDrive {
    Caption: Option<String>,
    SerialNumber: Option<String>,
    Model: Option<String>,
    MediaType: Option<String>,
    Size: Option<String>,
}

pub fn list_disks_wmi() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32DiskDrive> = wmi_con.query()?;

    let mut disks = vec![];

    for drive in results {
        let name = drive.Caption.unwrap_or_else(|| "Unknown".to_string());
        let model = drive.Model.unwrap_or_else(|| "Unknown".to_string());
        let serial = drive.SerialNumber.unwrap_or_else(|| "Unknown".to_string());
        let media_type = drive.MediaType.unwrap_or_else(|| "Unknown".to_string());

        let is_removable = media_type.to_lowercase().contains("removable");

        let size_gb = drive.Size
            .and_then(|s| s.parse::<u64>().ok())
            .map(|bytes| bytes / 1_000_000_000)
            .unwrap_or(0);


        disks.push(DiskInfo {
            name,
            size_gb,
            is_removable,
            model: Some(model),
            serial: Some(serial),
        });

    }

    Ok(disks)
}


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
                    model: None,
                    serial: None,
                });

            }
            _ => {
                // نادیده بگیر
            }
        }
    }

    Ok(disks)
}

pub fn get_disks_info_wmi() -> Result<Vec<DiskExtraInfo>, Box<dyn std::error::Error>> {
       let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_DiskDrive")?;

    let mut infos = Vec::new();

    for res in results {
        
        let device_id = res.get("DeviceID").and_then(|v| match v {
               Variant::String(s) => Some(s.clone()),
                    _ => None,
            }).unwrap_or_default();
        let model = res.get("Model").and_then(|v| match v {
                Variant::String(s) => Some(s.clone()),
                _ => None,
            }).unwrap_or_default();

        let serial = res.get("SerialNumber").and_then(|v| match v {
                Variant::String(s) => Some(s.clone()),
                _ => None,
            }).unwrap_or_default();

        let is_removable = res.get("MediaType").map(|v| match v {
                Variant::String(s) => s.contains("Removable"),
                _ => false,
            }).unwrap_or(false);


        infos.push(DiskExtraInfo {
            device_id,
            model: Some(model),
            serial: Some(serial),
            is_removable,
        });
    }

    Ok(infos)
}


pub fn list_disks_combined() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    let mut disks = vec![];

    let wmi_info = get_disks_info_wmi()?; // تابعی که فقط اطلاعات مدل، سریال و removable میده

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

        if let Ok(h) = handle {
            if h.0 == INVALID_HANDLE_VALUE.0 {
                continue;
            }

            let size = get_disk_size(h).unwrap_or(0);
            unsafe { let _ = CloseHandle(h); }

            // جستجوی اطلاعات اضافی از wmi
           let extra = wmi_info.iter().find(|info| path.contains(&info.device_id));

            disks.push(DiskInfo {
                name: path.clone(),
                size_gb: size / 1_000_000_000,
                is_removable: extra.map_or(false, |d| d.is_removable),
                model: extra.and_then(|d| d.model.clone()),
                serial: extra.and_then(|d| d.serial.clone()),
            });
        }
    }

    Ok(disks)
}
