pub mod field;
pub mod memtable;
pub mod meta;
pub mod meta_db;
pub mod model;
pub mod record;
pub mod timestamp;
pub mod wal;
pub mod btree;


// define macrocosmdb's version.
pub const VERSION: &str = "1.0.0";

// define macrocosmdb's api

// open a macrocosmdb database.
pub fn open() -> Result<(), ()> {
    todo!()
}

// close a macrocosmdb database.
pub fn close() -> Result<(), ()> {
    todo!()
}

// create a macrocosmdb database.
pub fn create() -> Result<(), ()> {
    todo!()
}   

// drop a macrocosmdb database.
pub fn drop() -> Result<(), ()> {
    todo!()
}