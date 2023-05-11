#![allow(warnings, unused)]
use super::field::{Field, Timestamp};
use super::record::Record;
use skiplist::skipmap::SkipMap;
use core::time;
use std::collections::LinkedList;

pub type MemTable = MemTable_<SkipMap<Field, (Vec<Field>, RecInstruct)>>;

macro_rules! check_state {
    ($x1 : expr, $x2 : expr, $lexp : expr, $rexp : expr, $ret : expr ) => {
        if $x1 == $lexp{
            if $x2 == $rexp{
                return $ret;
            }
        }
    };
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecInstruct{
    Insert = 1 << 0,
    Update = 1 << 1,
    Delete = 1 << 2,
    Put    = 1 << 3,
    Remove = 1 << 4,
    Null   = 1 << 5,
}

impl std::ops::BitOr for RecInstruct {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs_bits = self as u8;
        let rhs_bits = rhs as u8;
        let result_bits = lhs_bits | rhs_bits;
        match result_bits{
            0x1 => RecInstruct::Insert,
            0x2 => RecInstruct::Update,
            0x4 => RecInstruct::Delete,
            0x8 => RecInstruct::Put,
            0x10 => RecInstruct::Remove,
            0x20 => RecInstruct::Null,
            _ => panic!("Invalid bit pattern"),
        }
    }
}

fn linkedlist_to_vec<T: Clone>(list: LinkedList<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for item in list {
        vec.push(item.clone());
    }
    vec
}

// struct v<T>{
    
// }

trait MemStroage{

    fn insert(&mut self, timestamp : Field, values : Vec<Field>, ins : RecInstruct) -> usize;


    fn get(&self, timestamp : &Field) -> Option<&(Vec<Field>, RecInstruct)>;

    fn remove(&mut self, timestamp : &Field) -> bool;

    fn exist(&self, fd : &Field) -> bool;
}

impl MemStroage for SkipMap<Field, (Vec<Field>, RecInstruct)>{
    fn insert(&mut self, timestamp : Field, values : Vec<Field>, ins : RecInstruct) -> usize
    {
        SkipMap::insert(self, timestamp, (values, ins));
        0
    }

    fn get(&self, timestamp : &Field) -> Option<&(Vec<Field>, RecInstruct)> {
        match SkipMap::get(self, timestamp){
            Some(x) => Some(&(x)),
            None => None,
        }
    }

    fn remove(&mut self, timestamp : &Field) -> bool {
        if !self.exist(timestamp){
            return false;
        }
        SkipMap::remove(self, timestamp);
        true
    }

    fn exist(&self, fd : &Field) -> bool {
        SkipMap::contains_key(self, &fd);
        true
    }
}

struct MemTable_<T>{
    store           : T    ,
    threshold_front : usize, 
    threshold_end   : usize,  
    n_rows          : usize,  
}

impl<T> MemTable_<T>
where T : MemStroage,
{
    fn is_in_history<F>(&mut self, closure : F, timestamp : Field) -> bool
    where
        F : Fn(Field) -> bool,
    {
        closure(timestamp)
    }

    fn is_in_mem<F>(&self, closure : F) -> bool
    where
        F : Fn() -> bool,
    {
        closure()
    }

    fn check_threshold_front(&self) -> bool{
        self.n_rows + 1 > self.threshold_front
    }

    fn check_threshold_end(&self) -> bool{
        self.n_rows + 1 > self.threshold_end
    }

    fn get_option(mut values : &(Vec<Field>, RecInstruct)) -> RecInstruct{
        match values.1 {
            RecInstruct::Insert => RecInstruct::Insert,
            RecInstruct::Delete => RecInstruct::Delete,
            RecInstruct::Remove => RecInstruct::Remove,
            RecInstruct::Update => RecInstruct::Update,
            RecInstruct::Put    => RecInstruct::Put,
            RecInstruct::Null   => RecInstruct::Null,
        }
    }

    fn put_option(opt : Field, mut values : Vec<Field>) -> bool{
        if values.is_empty(){
            return false;
        }
        values[0] = opt;
        true
    }

    /// doc/storage/memtable_design.md
    fn state_transition(x1 : RecInstruct, x2 : RecInstruct) -> Option<RecInstruct> {
        // Insert x Insert => Failed x
        // Insert x Update => Put
        // Insert x Delete => NULL
        // Insert x Put => Put
        // Insert x Remove => NULL

        check_state!(x1, x2, RecInstruct::Insert, RecInstruct::Insert, None);
        check_state!(x1, x2, RecInstruct::Insert, RecInstruct::Update, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Insert, RecInstruct::Delete, Some(RecInstruct::Null));
        check_state!(x1, x2, RecInstruct::Insert, RecInstruct::Put, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Insert, RecInstruct::Remove, Some(RecInstruct::Remove));

        // Update x Insert => Failed x
        // Update x Update => Put
        // Update x Delete => Remove 
        // Update x Put => Put
        // Update x Remove => Remove 
        check_state!(x1, x2, RecInstruct::Update, RecInstruct::Insert, Some(RecInstruct::Insert));
        check_state!(x1, x2, RecInstruct::Update, RecInstruct::Update, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Update, RecInstruct::Delete, Some(RecInstruct::Remove));
        check_state!(x1, x2, RecInstruct::Update, RecInstruct::Put, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Update, RecInstruct::Remove, Some(RecInstruct::Remove));

        // Delete x Insert => Put
        // Delete x Update => Failed x
        // Delete x Delete => Failed x
        // Delete x Put => Put
        // Delete x Remove => Remove
        check_state!(x1, x2, RecInstruct::Delete, RecInstruct::Insert, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Delete, RecInstruct::Update, None);
        check_state!(x1, x2, RecInstruct::Delete, RecInstruct::Delete, None);
        check_state!(x1, x2, RecInstruct::Delete, RecInstruct::Put, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Delete, RecInstruct::Remove, Some(RecInstruct::Remove));

        // Put x Insert => Failed
        // Put x Update => Put
        // Put x Delete => Remove
        // Put x Put => Put
        // Put x Remove => Remove

        check_state!(x1, x2, RecInstruct::Put, RecInstruct::Insert, None);
        check_state!(x1, x2, RecInstruct::Put, RecInstruct::Update, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Put, RecInstruct::Delete, Some(RecInstruct::Remove));
        check_state!(x1, x2, RecInstruct::Put, RecInstruct::Put, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Remove, RecInstruct::Remove, Some(RecInstruct::Remove));

        // Remove x Insert => Put
        // Remove x Update => Failed
        // Remove x Delete => Failed
        // Remove x Put => Put
        // Remove x Remove => Remove
        check_state!(x1, x2, RecInstruct::Remove, RecInstruct::Insert, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Remove, RecInstruct::Update, None);
        check_state!(x1, x2, RecInstruct::Remove, RecInstruct::Delete, None);
        check_state!(x1, x2, RecInstruct::Remove, RecInstruct::Put, Some(RecInstruct::Put));
        check_state!(x1, x2, RecInstruct::Remove, RecInstruct::Remove, Some(RecInstruct::Remove));

        None
    }

    fn insert<F>(&mut self, rec : Record, closure : F) -> bool
    where
        F : Fn(Field) -> bool,
    {   
        let mut values = Vec::new();
        let target = rec.split_kv();

        for item in target.1{
            values.push(item);
        }

        if !self.is_in_mem(||self.store.exist(&target.0)) &&
           !self.is_in_history(closure, target.0)
        {
            self.store.insert(target.0, values, RecInstruct::Insert);
            return true;
        }else{
            return false;
        }
    }

    fn update<F>(&mut self, rec : Record, closure : F) -> bool 
    where
        F : Fn(Field) -> bool,
    {
        let mut values = Vec::new();
        let target = rec.split_kv();
    
        for item in target.1{
            values.push(item);
        }

        if self.is_in_mem(||self.store.exist(&target.0)){
            // ┌─────────┬  ┬──────┬──────┬───┐
            // │Timestamp│  │option│values│...│
            // └─────────┴, ┴──────┴──────┴───┘
            let mut mem_rec = self.store.get(&target.0);
            match mem_rec{
                Some(x) => {
                    match Self::state_transition(Self::get_option(x), RecInstruct::Update){
                        Some(x) if x == RecInstruct::Put => {
                            self.store.remove(&target.0);
                            self.store.insert(target.0, values, RecInstruct::Put);
                            return true;
                        }
                        Some(_) => {
                            return false;
                        },
                        None => {
                            return false;
                        },
                    }
                },
                None => {
                    return false;
                },
            }
        }

        if self.is_in_history(closure, target.0){
            let mut rec = Vec::new();
            self.store.insert(target.0, rec, RecInstruct::Update);
        }

        true
    }

    fn delete<F>(&mut self, timestamp : Field, closure : F) -> bool
    where
        F : Fn(Field) -> bool,
    {
        if self.is_in_mem(||self.store.exist(&timestamp)){
            return self.store.remove(&timestamp);
        }

        if self.is_in_history(closure, timestamp){

            let mut values = Vec::new();
            self.store.insert(timestamp, values, RecInstruct::Delete);
        }
        true
    }

    fn put(&mut self, rec : Record) -> bool{
        let mut values = Vec::new();
        let target = rec.split_kv();
    
        for item in target.1{
            values.push(item);
        }

        let mem_rec = self.store.get(&target.0);
        match mem_rec {
            Some(x) => {
                let mut rec = Vec::new();
                self.store.insert(target.0, rec, RecInstruct::Put);
            },
            None => { return false;}
        }
        self.store.insert(target.0, values, RecInstruct::Put);
        true
    }

    fn remove<F>(&mut self, timestamp : Field, closure : F) -> bool
    where 
        F : Fn(Field) -> bool,
    {
        if self.is_in_mem(||self.store.exist(&timestamp)){
            self.store.remove(&timestamp)
        }
        // else if self.is_in_history(closure, timestamp){
            
        // }
        else{
            return false;
        }
    }
    fn merge() -> usize{
        0
    }
}
