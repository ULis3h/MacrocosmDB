//! WAL records all updates and modifications related to the database.
//! A preliminary idea is to enable WAL to be compressed.
//!

#![allow(warnings, unused)]
use std::path::PathBuf;
use util::getters_generate;

/// Wal Record's Type.
/// This may be Meta and Normal table 's DDL and DML.
/// For each type, a WAL file named after the operation type is generated.
/// For the reason for such division, similar operations theoretically have
/// the same appearance, which is very helpful for compressing data.
#[derive(Clone, Copy)]
pub enum WalRecordType {
  /// DDL, When Create Meta table Or Normal table, write this type wal.
  Create,

  /// Insert Data for table write this type wal.
  Insert,

  /// When Update table's.
  Update,

  /// When Delete Data, write this type wal.
  Delete,
}

/// Structure for writing WAL files to disk.
pub struct WalWriter {
  /// Current write wal's type.
  wal_type: WalRecordType,

  /// Wal file save path.
  path: PathBuf,
}

impl WalWriter {
  // generate get_wal for WalWriter.
  getters_generate!(get_wal, wal_type, WalRecordType);
}

pub struct WalWriterBuilder {
  wal_type: WalRecordType,
  path: PathBuf,
}

impl WalWriterBuilder {
  /// assignment for wal_type.
  fn wal_type(&mut self, p_wal_type: WalRecordType) -> &mut Self {
    self.wal_type = p_wal_type;
    self
  }

  /// set a path for WalWriter.
  fn path(&mut self, p_path: PathBuf) -> &mut Self {
    self.path = p_path;
    self
  }

  /// return a WalWriter object.
  fn build(&self) -> WalWriter {
    WalWriter {
      wal_type: self.wal_type,

      /// This wastes performance, but currently I don't know how to handle it.
      path: self.path.clone(),
    }
  }
}
