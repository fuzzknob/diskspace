use byte_unit::rust_decimal::prelude::ToPrimitive;
use std::ops::{Div, Mul};
use sysinfo::Disks;

#[derive(Debug)]
pub struct DiskInfo {
    pub drive_name: String,
    pub used_percent: u16,
    pub total: String,
    pub used: String,
    pub available: String,
}

pub fn get_disk_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|disk| {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let percentage = get_used_percent(used_space, total_space);
            let mount_path = disk.mount_point().to_str().unwrap();
            DiskInfo {
                used_percent: percentage,
                drive_name: get_drive_name(mount_path),
                total: format_size(total_space),
                available: format_size(available_space),
                used: format_size(used_space),
            }
        })
        .collect::<Vec<_>>()
}

const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
const DIFFERENCE: f64 = 1024.0;

fn format_size(bytes: u64) -> String {
    if bytes == 0 {
        return String::from("0 B");
    }
    let target = bytes.to_f64().unwrap();
    let power = target.log(DIFFERENCE).floor();
    let index = power.to_usize().unwrap();
    let unit = UNITS.get(index).unwrap_or(&"NULL");
    let size = target.div(DIFFERENCE.powf(power));
    return format!("{:.2} {}", size, unit);
}

fn get_used_percent(available_bytes: u64, total_bytes: u64) -> u16 {
    let available = available_bytes.to_f64().unwrap();
    let total = total_bytes.to_f64().unwrap();
    return (available.div(total)).mul(100.0) as u16;
}

fn get_drive_name(mount_path: &str) -> String {
    if mount_path == "/" {
        return "root".to_string();
    }
    mount_path
        .split("/")
        .collect::<Vec<_>>()
        .last()
        .unwrap_or(&"")
        .to_string()
}
