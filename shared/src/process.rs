
// pub enum PermissionFlags {
//     Read = 1 << 0,
//     Write = 1 << 1,
//     Execute = 1 << 2,
//     Shared = 1 << 3,
//     Private = 1 << 4,
// }

// pub struct Region {
//     pub start: u64,
//     pub end: u64,
//     pub size: u64,
//     pub permissions: u32,
//     pub offset: u64,
//     pub device: u64,
//     pub inode: u64,
//     pub pathname: String,
// }

// pub struct Process {
//     pub pid: i32,
//     pub name: String,
// }

use crate::protocol::{Region, ProcessEntry};

pub fn get_regions(pid: i32) -> Vec<Region> {
    todo!("Implement me!")
}
pub fn get_running_processes() -> Vec<ProcessEntry> {
    todo!("Implement me!")
}