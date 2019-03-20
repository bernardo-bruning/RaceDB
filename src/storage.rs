use std::io::Read;
use std::io::Write;
use std::io::Seek;
use crate::pagination::Page;

pub fn allocate<RW: Read+Write+Seek>(target:RW, source: Vec<Page>) -> (RW, u32)
{
  (target, 0)
}