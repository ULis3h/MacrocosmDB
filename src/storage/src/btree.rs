/*
 * MacrocosmDB
 * btree.rs
 * 
 * Copyright (c) 2024 ULis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 * Description: Brief description of what this file is for.
 */

enum Node<K, V> {
    Internal(InternalNode<K, V>),
    Leaf(LeafNode<K, V>),
}

struct InternalNode<K, V> {
    keys : Vec<K>,
    children : Vec<Box<Node<K, V>>>,
}

struct LeafNode<K, V> {
    keys : Vec<K>,
    values : Vec<V>,
    next : Option<Box<LeafNode<K, V>>>,
}

// B+ tree structure
pub struct BPTree<K, V> {
    root: Option<Box<Node<K, V>>>,
    order: usize,
}

impl<K: Ord + Clone, V> BPTree<K, V> {
    // Create a new B+ tree
    pub fn new(order: usize) -> Self {
        BPTree {
            root: None,
            order,
        }
    }

    // Insert a key-value pair
    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            let mut leaf = LeafNode {
                keys: vec![key],
                values: vec![value],
                next: None,
            };
            self.root = Some(Box::new(Node::Leaf(leaf)));
        }
        // More complex insertion logic will be implemented later
    }

    // Get the value associated with a key
    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.root {
            Some(node) => {
                match &**node {
                    Node::Leaf(leaf) => {
                        if let Some(pos) = leaf.keys.iter().position(|k| k == key) {
                            Some(&leaf.values[pos])
                        } else {
                            None
                        }
                    }
                    Node::Internal(_) => None, // Will be implemented later
                }
            }
            None => None,
        }
    }

    // Delete a key-value pair
    pub fn delete(&mut self, key: &K) {
        if let Some(node) = &mut self.root {
            if self.order < 2 {
                return; // Invalid order
            }
            // Basic deletion logic - more complex logic will be implemented later
            match &mut **node {
                Node::Leaf(leaf) => {
                    if let Some(pos) = leaf.keys.iter().position(|k| k == key) {
                        leaf.keys.remove(pos);
                        leaf.values.remove(pos);
                    }
                }
                Node::Internal(_) => {} // Will be implemented later
            }
        }
    }
}

// Helper functions for node operations
impl<K: Ord + Clone, V> Node<K, V> {
    // ... (Implementation for node splitting, merging, etc.)
}