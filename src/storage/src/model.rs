#![allow(warnings, unused)]

#[cfg(not(feature = "mcdb_map"))]
use std::collections::HashMap;
use crate::timestamp;


/// A data point
pub struct Point{
    timestamp  : String,
    metric     : Metric,
    datasource : Datasource,
}

pub struct Metric{
    degree : HashMap<String, String>
}

/// Datasource include some tags.
pub struct Datasource{
    tags : HashMap<String, String>,
}