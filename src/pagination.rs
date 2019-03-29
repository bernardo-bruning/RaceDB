use crate::serialization::Serializable;
use crate::serialization::DeserializationError;
use std::vec::Vec;
use std::convert::From;
use std::iter::*;
use std::io::Error;
use std::slice::Iter;

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
    //Contains page ID, this data is not serialize to not consume space
    pub id: u32,
    // Contains ID for next page
    pub next: u32,
    // Represent size of the content
    pub size: u32,
    // Represent content of the page
    pub content: Vec<u8>
}

impl Page {
    fn extract_content(&self) -> Vec<u8>
    {
        self.content.to_owned()
    }
}

pub struct Pages{
    page_size: usize,
    content: Vec<Page>
}

impl Pages {
    pub fn from<TSerializable>(object: &TSerializable, page_size: usize) -> Self
        where TSerializable: Serializable
    {
        Pages {
            page_size: page_size,
            content: paginate(object, page_size)
        }
    }

    pub fn first(&self) -> Option<&Page>
    {
        self.content.first()
    }

    pub fn len(&self) -> usize
    {
        self.content.len()
    }

    pub fn last(&self) -> Option<&Page>
    {
        self.content.last()
    }
    pub fn iter(&self) -> Iter<Page> {
        self.content.iter()
    }
}

pub fn fill<T>(source: &Vec<T>, default: T, size: usize) -> Vec<T> where T: Clone {
    let diff_size = size - source.len();
    let fill = (0..diff_size).map(|x| default.clone()).collect();
    let source_filled = [source.clone(), fill];
    source_filled.concat()
}

fn paginate<TSerializable>(data: &TSerializable, size: usize) -> Vec<Page> 
        where TSerializable : Serializable
{
    let bytes_data: Vec<u8> = data.serialize();
    bytes_data
        .chunks(size as usize)
        .map(|x| Page {
            id: 0,
            next: 0,
            size: x.len() as u32,
            content: fill(&x.to_vec(), 0, size as usize)
        })
        .collect()
}

fn are_ordered_pages(pages: &[Page]) -> bool
{
    true
}

pub fn mount_data<TSerializable>(pages: &[Page]) -> Result<TSerializable, DeserializationError> 
    where TSerializable: Serializable
{
    debug_assert!(are_ordered_pages(pages));
    let content = pages
        .iter()
        .map(|x| x.extract_content())
        .collect::<Vec<Vec<u8>>>()
        .concat();

    TSerializable::deserialize(&content)
}

fn collect_u32(from: &[u8], starting: usize) -> u32{
        let mut value: [u8; 4] = [0;4];
        let value_arr: Vec<u8> = from
            .iter()
            .skip(starting)
            .take(4)
            .map(|x| x.to_owned())
            .collect();
        value.copy_from_slice(&value_arr);
        convert_u8_to_u32(&value)
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
        let content: Vec<u8> = bytes
            .to_vec()
            .iter()
            .skip(8)
            .map(|x| x.to_owned())
            .collect();
        
        Result::Ok(Page {
            id: 0,
            size: collect_u32(bytes, 0),
            next: collect_u32(bytes, 4),
            content: content
        })
    }
}

impl From<&[u8]> for Page {
    fn from(value: &[u8]) -> Self
    {
        let size = value.len() as u32;
        Page {
            id: 0,
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
            id: 0,
            next: 0,
            size: size,
            content: value.as_bytes().to_vec()
        }
    }
}