#![allow(warnings, unused)]
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum TimeStampPrecision {
  Nanoseconds,
  Microseconds,
  Milliseconds,
  Seconds,
}
