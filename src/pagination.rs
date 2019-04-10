use crate::serialization::Serializable;
use crate::serialization::DeserializationError;
use crate::serialization::to_option;
use std::vec::Vec;
use std::convert::From;
use std::iter::*;
use std::vec::IntoIter;
use std::slice::Iter;

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
    pub id: usize,
    // Contains ID for next page
    pub next: Option<u32>,
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

    fn is_free(&self) -> bool
    {
        self.id == 0
    }

    pub fn get_bytes_size(size: usize) -> usize
    {
        let page_offset = 2*4;
        (page_offset + size)
    }

    pub fn set_id(mut self, id: usize) -> Self
    {
        self.id = id;
        self
    }
}

pub struct Pages{
    pub page_size: usize,
    content: Vec<Page>
}

impl Pages {
    pub fn new(page_size: usize, content: Vec<Page>) -> Pages
    {
        Pages {
            page_size: page_size,
            content: content
        }
    }

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

    pub fn into_iter(self) -> IntoIter<Page> {
        self.content.into_iter()
    }

    pub fn get_byte_size(&self) -> usize
    {
        (Page::get_bytes_size(self.page_size) * self.len())
    }
}

impl Serializable for Pages {
    fn serialize(&self) -> Vec<u8>
    {
        let content: Vec<u8> = self
            .iter()
            .flat_map(|x| x.serialize())
            .collect();
        [self.page_size.serialize(), content].concat()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized
    {
        
        Result::Ok(Pages{
            page_size: 0,
            content: Vec::new()
        })
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
            next: Option::None,
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

fn collect_u32_bytes(from: &[u8], starting: usize) -> Vec<u8> {
    let value_arr: Vec<u8> = from
        .iter()
        .skip(starting)
        .take(4)
        .map(|x| x.to_owned())
        .collect();
    return value_arr
}

fn collect_u32(from: &[u8], starting: usize) -> Result<u32, DeserializationError>{
    u32::deserialize(&collect_u32_bytes(from, starting))
}

impl Serializable for Page {
    fn serialize(&self) -> Vec<u8>
    {
        let size:Vec<u8> = self.size.serialize();
        let next:Vec<u8> = self.next.serialize();
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

        match (collect_u32(bytes, 0), collect_u32(bytes, 4)){
            (Ok(size), Ok(next)) => 
                Ok(Page {
                    id: 0,
                    size: size,
                    next: to_option(next),
                    content: content
                }),
            _ => Result::Err(DeserializationError::new())
        }
    }
}

impl From<&[u8]> for Page {
    fn from(value: &[u8]) -> Self
    {
        let size = value.len() as u32;
        Page {
            id: 0,
            next: Option::None,
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
            next: Option::None,
            size: size,
            content: value.as_bytes().to_vec()
        }
    }
}
