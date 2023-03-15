#![allow(warnings, unused)]

#[cfg(not(feature = "mcdb_map"))]
use std::collections::HashMap;
use crate::timestamp;


/// A data point
#[derive(Debug, Clone)]
pub struct Point{
    pub timestamp  : String,
    pub metric     : Metric,
    pub datasource : Datasource,
}

/// map represents the dimension and its value.
#[derive(Debug, Clone)]
pub struct Metric{
    degree : HashMap<String, String>,
}

/// Datasource include some tags.
#[derive(Debug, Clone)]
pub struct Datasource{
    tags : HashMap<String, String>,
}