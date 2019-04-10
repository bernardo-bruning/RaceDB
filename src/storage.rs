use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use crate::pagination::Pages;
use crate::pagination::Page;
use crate::serialization::Serializable;
use std::io::Error;

pub trait Storable: Sized+Serializable {
  fn store(self, store: &mut Store) -> Result<Self, Error>;
}

pub trait Store: Read+Write+Seek {}

impl <TStore: Read+Write+Seek> Store for TStore {}

impl Storable for Page {
  fn store(self, store: &mut Store) -> Result<Self, Error>
  {
    let source_serialized: Vec<u8> = self.serialize();
    let current_position = store.seek(SeekFrom::Current(0));
    let result = store.write(&source_serialized);
    
    match (current_position, result) {
      (Ok(position), Ok(_)) => Ok(self.set_id(position as usize)),
      (Ok(_), Err(write_bytes)) => Err(write_bytes),
      (Err(error_seek), _) => Err(error_seek)
    }
  }
}

impl Storable for Pages {
  fn store(self, store: &mut Store) -> Result<Self, Error>
  {
    let source_serialized: Vec<u8> = self.serialize();
    let page_size = self.page_size;
    let result: Result<Vec<Page>, Error> = self
      .into_iter()
      .map(|x| x.store(store))
      .collect();
      
    match result {
      Ok(pages) => Ok(Pages::new(page_size, pages)),
      Err(error) => Err(error)
    }
  }
}