use std::io::Read;
use std::io::Write;
use std::io::Seek;
use crate::pagination::Page;
use crate::pagination::Pages;
use crate::serialization::Serializable;
use std::io::Error;

pub fn allocate<RW: Read+Write+Seek>(target:&mut RW, source: Pages) -> Result<u32, Error>
{
  let source_serialized: Vec<u8> = source
    .iter()
    .flat_map(|x| x.serialize())
    .collect();

  let result = target.write(&source_serialized);
  match result {
    Ok(_) => Ok(0),
    Err(error) => Err(error)
  }
}