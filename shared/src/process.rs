use proc_maps::*;

use crate::protocol::{Region, ProcessEntry, EncodedString};

pub fn get_regions(pid: i32) -> std::io::Result<Vec<Region>> {
    let maps = get_process_maps(pid)?;

    Ok(maps.iter()
        .map(|map_range| Region {
            start: map_range.start() as u64,
            end: map_range.start() as u64 + map_range.size() as u64,
            size: map_range.size() as u64,
            permissions: permissions_to_u32(map_range.flags.as_str()),
            offset: map_range.offset as u64,
            device: EncodedString::new(map_range.dev.clone()),
            inode: map_range.inode as u64,
            pathname: EncodedString::new(map_range.filename().unwrap().to_str().unwrap().to_string()),
        })
        .collect())
}

pub fn get_running_processes() -> std::io::Result<Vec<ProcessEntry>> {
    todo!("Implement me!")
}

const READ: u32 = 0b0001;      // 1 << 0
const WRITE: u32 = 0b0010;     // 1 << 1
const EXECUTE: u32 = 0b0100;   // 1 << 2
const PRIVATE: u32 = 0b1000;   // 1 << 3
// Add more flags as needed

fn permissions_to_u32(permissions: &str) -> u32 {
    let mut result = 0;

    for ch in permissions.chars() {
        match ch {
            'r' => result |= READ,
            'w' => result |= WRITE,
            'x' => result |= EXECUTE,
            'p' => result |= PRIVATE,
            // Handle other permissions as needed
            _ => {}
        }
    }

    result
}