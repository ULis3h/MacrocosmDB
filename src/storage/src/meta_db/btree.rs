#![allow(warnings, unused)]
use std::sync::Arc;

pub struct Btree(pub(crate) Arc<BtreeInner>);

pub struct BtreeInner {}
