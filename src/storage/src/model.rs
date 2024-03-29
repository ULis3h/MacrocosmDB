#![allow(warnings, unused)]

use crate::timestamp;
#[cfg(not(feature = "mcdb_map"))]
use std::collections::HashMap;

/// A series point
/// After the line protocol is parsed,
/// a slice of this structure should be returned
#[derive(Debug, Clone)]
pub struct Point {
  pub metric: Metric,
  pub datasource: Datasource,
  pub value: SeriesValue,
}

/// map represents the dimension and its value.
#[derive(Debug, Clone)]
pub struct Metric {
  degree: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SeriesValue {
  // timestamp <-> values
  pub values: HashMap<String, Vec<String>>,
}

/// Datasource include some tags.
#[derive(Debug, Clone)]
pub struct Datasource {
  tags: HashMap<String, String>,
}
