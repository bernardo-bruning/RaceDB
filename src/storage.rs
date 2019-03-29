use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::Cursor;
use crate::pagination::Pages;
use crate::serialization::Serializable;
use std::io::Error;

pub trait Storable: Sized+Serializable {
  fn store(self, store: &mut Store) -> Result<Self, Error>;
}

pub trait Store: Read+Write+Seek {}

impl Store for Cursor<Vec<u8>> {}

impl Storable for Pages {
  fn store(self, store: &mut Store) -> Result<Self, Error>
  {
    let source_serialized: Vec<u8> = self.serialize();
    let result = store.write(&source_serialized);
    match result {
      Ok(_) => Ok(self),
      Err(error) => Err(error)
    }
  }
}