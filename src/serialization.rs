use std::marker::Sized;

#[derive(Debug)]
pub struct DeserializationError
{
    error: String
}

impl DeserializationError {
    pub fn new() -> Self {
        DeserializationError {
            error: String::from("Error to deserialize string!")
        }
    }
}

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized;
}

impl Serializable for u32 {
    fn serialize(&self) -> Vec<u8>
    {
        let b1 : u8 = ((self >> 24) & 0xff as u32) as u8;
        let b2 : u8 = ((self >> 16) & 0xff as u32) as u8;
        let b3 : u8 = ((self >> 8) & 0xff as u32) as u8;
        let b4 : u8 = (self & 0xff) as u8;
        [b1, b2, b3, b4].to_vec()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized
    {
        Result::Ok(
        ((bytes[0] as u32) << 24) +
        ((bytes[1] as u32) << 16) +
        ((bytes[2] as u32) <<  8) +
        ((bytes[3] as u32) <<  0))
    }
}

impl Serializable for usize {
    fn serialize(&self) -> Vec<u8>
    {
        (self.to_owned() as u32).serialize()
    }
    
    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized
    {
        u32::deserialize(bytes).map(|x| x as usize)
    }
}

impl Serializable for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError>
    {
        match String::from_utf8(bytes.to_vec()) {
            Ok(value) => Result::Ok(value),
            _ => Result::Err(DeserializationError::new())
        }
    }
}