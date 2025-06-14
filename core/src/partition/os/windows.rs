use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};
use crate::partition::provider::PartitionInfo;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Win32_LogicalDisk {
    DeviceID: String,
    VolumeName: Option<String>,
    FileSystem: Option<String>,
    Size: Option<String>,
    FreeSpace: Option<String>,
    DriveType: u32,
}

pub fn list_partitions() -> Result<Vec<PartitionInfo>, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32_LogicalDisk> = wmi_con.query()?;

    let partitions = results
        .into_iter()
        .map(|p| {
            let size_bytes = p.Size.as_deref().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let free_bytes = p.FreeSpace.as_deref().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let used_bytes = size_bytes.saturating_sub(free_bytes);

            PartitionInfo {
                name: Some(p.DeviceID.clone()),
                volume_label: Some(p.VolumeName.unwrap_or_default()),
                file_system: Some(p.FileSystem.unwrap_or_else(|| "Unknown".into())),
                total_space: size_bytes,
                free_space: free_bytes,
                used_space: Some(used_bytes),
                is_removable: p.DriveType == 2,
                mount_point: p.DeviceID.clone(), // اگر mount_point = DeviceID باشد
            }
        })
        .collect();

    Ok(partitions)
}
