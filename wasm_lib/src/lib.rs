use wasm_bindgen::prelude::*;
use shared::protocol::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn parse_payload_to_string(msg: &[u8]) -> String {
    if msg.is_empty() {
        return "Empty message".to_string();
    }
    match PacketType::from_u8(msg[0]) {
        Some(PacketType::Read) => {
            let packet = S2CReadMemoryPacketResponse::parse(&msg);
            format!("Read: count: {}, data: {:?}", packet.count, packet.data)
        },
        Some(PacketType::Write) => {
            let packet = S2CWriteMemoryPacketResponse::parse(&msg);
            format!("Write: bytes written: {}", packet.bytes_written)
        },
        Some(PacketType::TargetPID) => {
            let packet = S2CTargetPidRegionsPacket::parse(&msg);
            let regions_string = packet.regions.iter().fold(String::new(), |acc, region| {
                acc + &format!("Start: {}, End: {}, Size: {}, Permissions: {}, Offset: {}, Device: {}, Inode: {}, Pathname: {}\n", region.start, region.end, region.size, region.permissions, region.offset, region.device.to_string(), region.inode, region.pathname.to_string())
            });
            format!("TargetPID: count: {}, regions: {}\n", packet.count, regions_string)
        },
        Some(PacketType::SendProcesses) => {
            let packet = S2CSendProcessesPacket::parse(&msg);
            let processes = packet.processes.iter().fold(String::new(), |acc, process| {
                acc + &format!("Pid: {}, Name: {}\n", process.pid, process.name.to_string())
            });
            format!("SendProcesses: count: {} processes: {}\n", packet.count, processes)
        },
        None => {
            format!("None")
        }
    }
}

#[wasm_bindgen]
pub fn target_pid_packet_data(pid: i32) -> Vec<u8> {
    C2STargetPidPacket::out_bytes(pid)
}

#[wasm_bindgen]
pub fn read_memory_packet_data(address: u64, size: u32) -> Vec<u8> {
    C2SReadMemoryPacket::out_bytes(address, size)
}

#[wasm_bindgen]
pub fn write_memory_packet_data(address: u64, bytes: &[u8]) -> Vec<u8> {
    C2SWriteMemoryPacket::out_bytes(address, bytes.to_vec())
}

#[wasm_bindgen]
pub fn get_processes_packet_data() -> Vec<u8> {
    C2SGetProcessesPacket::out_bytes()
}