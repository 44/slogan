#[repr(packed)]
pub struct Header {
    magic: u64, // 8
    version: u32, // 12
    capabilities: u32, // 16
    flags: u32, // 20
    platform: u32, // 24
    image_type: u32, // 28
    image_version: [u8; 64], // 92
    platform_version: [u8; 64], // 156
    reserved: [u8; 100], // 256
}

pub fn inspect_log_file(path: &std::path::Path) {
}
