use crate::serialization::Serializable;
use crate::serialization::DeserializationError;
use std::vec::Vec;
use std::convert::From;
use std::iter::*;
use std::io::Error;

/// Page is a strucuture when enable fast write and read data, creating pages like a book in database.
/// Pages has a size and reference for next page with also your content data.
/// In of the file database has the structure data:
/// 
/// +-----------+
/// |Part data A|
/// |-----------|
/// |Part data A|
/// |-----------|
/// |Part data B|
/// |-----------|
/// |Part data A|
/// +-----------+
/// 
/// From this diagram the Data A is slice into 3 parts and Data B in one part, 
/// from this database file create defragmented information withou rewrite all file when updated one data.
#[derive(Debug)]
pub struct Page {
    pub size: u32,
    pub next: u32,
    pub content: Vec<u8>
}

pub fn fill<T>(source: &Vec<T>, default: T, size: usize) -> Vec<T> where T: Clone {
    let diff_size = size - source.len();
    let fill = (0..diff_size).map(|x| default.clone()).collect();
    let source_filled = [source.clone(), fill];
    source_filled.concat()
}

pub fn paginate<TSerializable>(data: TSerializable, size: u32) -> Vec<Page> 
        where TSerializable : Serializable
{
    let bytes_data: Vec<u8> = data.serialize();
    bytes_data
        .chunks(size as usize)
        .map(|x| Page {
            next: 0,
            size: size,
            content: fill(&x.to_vec(), 0, size as usize)
        })
        .collect()
}


pub fn mount_data<TSerializable>(pages: &[Page]) -> Result<TSerializable, DeserializationError> 
    where TSerializable: Serializable
{
    TSerializable::deserialize("".as_bytes())
}

fn convert_u8_to_u32(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}



fn convert_u32_to_u8(x: u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}

impl Serializable for Page {
    fn serialize(&self) -> Vec<u8>
    {
        let size:Vec<u8> = convert_u32_to_u8(self.size).to_vec();
        let next:Vec<u8> = convert_u32_to_u8(self.next).to_vec();
        let binary = vec![size, next, self.content.to_vec()];
        binary.concat()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized
    {        
        let mut size: [u8; 4] = [0;4];
        let size_arr: Vec<u8> = bytes.iter().take(4).map(|x| x.to_owned()).collect();
        size.copy_from_slice(&size_arr);

        let mut next: [u8; 4] = [0;4];
        let next_arr: Vec<u8> = bytes.iter().skip(4).take(4).map(|x| x.to_owned()).collect();
        next.copy_from_slice(&next_arr);

        let content: Vec<u8> = Vec::from(bytes).iter().skip(8).map(|x| x.to_owned()).collect();
        
        Result::Ok(Page {
            size: convert_u8_to_u32(&size),
            next: convert_u8_to_u32(&next),
            content: content
        })
    }
}

impl From<&[u8]> for Page {
    fn from(value: &[u8]) -> Self
    {
        let size = value.len() as u32;
        Page {
            next: 0,
            size: size,
            content: value.to_vec()
        }
    }
}

impl From<&str> for Page {
    fn from(value: &str) -> Self
    {
        let size = value.len() as u32;
        Page {
            next: 0,
            size: size,
            content: value.as_bytes().to_vec()
        }
    }
}