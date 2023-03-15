#![allow(warnings, unused)]
use std::cmp::Ordering;
use std::error::Error;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use crate::model::Point;
use crate::timestamp::TimeStampPrecision;

const MAX_FILE_SIZE : usize = 1 << 20; // 1MB.

#[derive(Clone, Debug)]
struct StroageConfig{
    default_partition_duration: Duration,
    default_retention: Duration,
    default_timestamp_precision: TimeStampPrecision,
    default_write_timeout: Duration,
    default_wal_buffered_size: usize,
    writable_partitions_num: usize,
    check_expired_interval: Duration,
    wal_dir_name: String,
}

impl Default for StroageConfig {
    fn default() -> StroageConfig {
        StroageConfig {
            default_partition_duration: Duration::from_secs(3600),
            default_retention: Duration::from_secs(1209600),
            default_timestamp_precision: TimeStampPrecision::Nanoseconds,
            default_write_timeout: Duration::from_secs(30),
            default_wal_buffered_size: 4096,
            writable_partitions_num: 2,
            check_expired_interval: Duration::from_secs(3600),
            wal_dir_name: String::from("wal"),
        }
    }
}

pub trait Storage{
    fn put(&mut self, data : Point) -> Result<(), String>;
}

pub struct LsmTree{
    // Data Struct inside Memory.
    mem_table : MemTable,
    level : Vec<Level>,
}

struct Level{
    /// notify thread safe and data consistency.
    sst : Vec<SSTable>,
}

struct SSTable{
    file   : File,
    offset : u64,
    index  : Vec<u64>,
}

impl SSTable {
    fn new(path : &Path) -> Result<Self, Box<dyn Error>>{
        let file = OpenOptions::new().read(true).open(path)?;
        let mut reader = BufReader::new(&file);
        let mut offset = 0;
        // let mut index = Btree

        while reader.seek(SeekFrom::Current(0)) ? < file.metadata()?.len(){

        }
        Ok(Self {
            file,
            offset: 0,
            index : Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
struct MemTable{
    inner : Vec<Point>,
    threshold : usize,
}

impl MemTable{
    fn new(threshold : usize) -> Self{
        Self { inner : Vec::new(), threshold,}
    }

    fn insert(&mut self, data : Point){
        self.inner.push(data);
        if self.inner.len() >= self.threshold{
            self.compact();
        }
    }

    fn get(&self, key : String) -> Option<Point>{
        // because data is sorted, use binary.
        let mut left = 0;
        let mut right = self.inner.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if self.inner[mid].timestamp == key { 
                return Some(self.inner[mid].clone());
            }else if self.inner[mid].timestamp < key{
                left = mid + 1;
            }else {
                right = mid - 1;
            }
        }
        None
    }

    fn iter(&self) -> impl Iterator<Item = &Point>{
        self.inner.iter()
    }

    fn compact(&mut self){
        let mut new_mem_table = MemTable::new(self.threshold);
        for p in self.inner.clone() {
            new_mem_table.insert(p);
        }
    }
}