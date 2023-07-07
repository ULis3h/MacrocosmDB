#![allow(warnings, unused)]
use super::memtable::RecInstruct;

pub type Timestamp = u64;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Field {
  I64(i64),
  // F64(f64),
  TimeStamp(Timestamp),
}

impl Field {
  #[inline]
  fn serialize(&self) -> Vec<u8> {
    todo!()
  }

  #[inline]
  fn serialize_into(&self, dst: &mut [u8]) -> usize {
    todo!()
  }

  #[inline]
  fn deserialize_from(&mut self, src: &[u8]) -> usize {
    todo!()
  }

  #[inline]
  fn deserialize(src: &[u8]) -> (usize, Self) {
    todo!()
  }
}
