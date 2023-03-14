#![allow(warnings, unused)]
use std::time::Duration;

use crate::timestamp::TimeStampPrecision;

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