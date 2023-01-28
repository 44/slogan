use std::mem;
use std::io::Read;
use std::io::prelude::*;
use flate2::read::GzDecoder;
use flate2::read::ZlibDecoder;
use std::io::SeekFrom;


#[allow(unaligned_references)]
#[derive(Debug, Clone)]
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

#[allow(unaligned_references)]
#[derive(Debug, Clone)]
#[repr(packed)]
pub struct EventHeader {
    magic: u32,
    event_type: u32,
    timestamp: u64,
    process_id: u32,
    thread_id: u32,
    guid: uuid::Uuid,
    event_id: u32,
    event_version: u32,
    payload_size: u32,
    trace_id: u32,
}

pub fn inspect_log_file(path: &std::path::Path) {
    println!("Reading {:?}", path);
    let mut header: Header;
    let f = std::fs::File::open(path).unwrap();
    let fsize = f.metadata().unwrap().len();
    let mut reader = std::io::BufReader::new(f);

    //let mut r = Vec::<Header>::with_capacity(1);
    let mut r: Header = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    unsafe {
        // let buffer = std::slice::from_raw_parts_mut(r.as_mut_ptr() as *mut u8, 256);
        {
        let buffer = std::slice::from_raw_parts_mut(&mut r as *mut _ as *mut u8, 256);
        reader.read_exact(buffer);
        let platform = r.platform;
        println!("Header: {}, {}, {}", platform, std::str::from_utf8(&r.image_version).unwrap(), std::str::from_utf8(&r.platform_version).unwrap());
        }
        println!("Full size: {}", fsize);
        let mut remaining = Vec::<u8>::with_capacity(fsize as usize - 256);
        reader.seek(SeekFrom::Start(256)).unwrap();
        let expected = reader.read_to_end(&mut remaining).unwrap();
        println!("expected: {}", expected);
        // remaining.set_len(fsize as usize - 256);
        println!("remaining: {}", &remaining.len());
        println!("bytes: {:?}", remaining);
        let mut d = GzDecoder::new(&*remaining); //"...".as_bytes());
        let mut extracted = Vec::<u8>::with_capacity(10000);
        d.read_to_end(&mut extracted);
        println!("Unzipped: {}", extracted.len());

        //r.set_len(1);

        // let buffer: &mut [u8] = std::slice::from_raw_parts_mut(
        //     &mut header as *mut _ as *mut u8, 256);
    }
    // println!("Header: {:?}, {}", r, std::str::from_utf8(&r[0].image_version).unwrap());
    // let buffer = std::fs::read(path);
}
