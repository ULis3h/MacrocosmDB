#![allow(warnings, unused)]
use std::collections::LinkedList;

use super::field::{Field, Timestamp};


/// fields
pub struct Record{
    fields    : LinkedList<Field>, 
}

impl Record {
    pub fn split_kv(mut self) -> (Field, LinkedList<Field>){
        (self.fields.pop_front().unwrap(), self.fields)
    }
}

impl PartialOrd for Record{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialEq for Record{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Record {
    fn new(items : impl Iterator<Item = Field>) -> Self{
        let mut lkst = LinkedList::new();
        for itr in items{
            lkst.push_back(itr);
        }
        Self { fields : lkst }
    }
    
}
// iter impl
struct RecordReader<'a>{
    fields: std::collections::linked_list::Iter<'a, Field>,
}

impl<'a> RecordReader<'a>{
    fn from_record<'b>(rcd: &'b Record)->Self
        where 'b: 'a
    {
        return RecordReader { fields: rcd.fields.iter()}
    }
}

impl<'a> Iterator for RecordReader<'a>{
    type Item = &'a Field;
    fn next(&mut self) -> Option<Self::Item> {
        self.fields.next()
    }
}