/*   
 * Copyright 2017 Clint Byrum
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;

/// A hash set that remembers the last key it returned with its iterator
/// it will wrap around and only return all of the keys once.
///
#[derive(Debug)]
pub struct WrappingHashSet<T>
    where T:Eq + Hash {
    hashset: HashSet<T>,
    keys: Vec<T>,
    pos: usize,
    count: usize,
}

pub struct Iter<'i, T: 'i>
    where T:Eq + Hash {
    whs: &'i mut WrappingHashSet<T>,
}

impl <'i, T>Iterator for Iter<'i, T>
    where T:Eq + Hash + Clone {
    type Item = T;
    fn next(&mut self) -> Option<T> {
       self.whs.pos += 1;
       self.whs.count += 1;
       if self.whs.count > self.whs.hashset.len() {
           self.whs.pos = 0;
           self.whs.count = 0;
           return None;
       }
       Some(self.whs.keys[self.whs.pos - 1].clone())
    }
}

impl <T>WrappingHashSet<T>
    where T:Eq + Hash + Clone {
    pub fn new() -> WrappingHashSet<T> {
        WrappingHashSet {
            hashset: HashSet::new(),
            keys: Vec::new(),
            pos: 0,
            count: 0,
        }
    }

    pub fn iter<'i>(&'i mut self) -> Iter<'i, T> {
        Iter {
            whs: self,
        }
    }

    pub fn insert(&mut self, key: T) -> bool {
        if self.hashset.insert(key.clone()) {
            self.keys.push(key);
            return true
        }
        return false
    }

    pub fn remove<'b>(&mut self, key: &'b T) -> bool {
        if self.hashset.remove(key) {
            self.keys = Vec::new();
            for k in self.hashset.iter() {
                self.keys.push(k.clone())
            }
            return true
        }
        return false
    }
}

#[test]
fn test_wrapping_hashset() {
    let mut hs: WrappingHashSet<&str> = WrappingHashSet::new();
    let mut keys_as_found: Vec<&str> = Vec::new();
    hs.insert("foo");
    hs.insert("bar");
    hs.insert("baz");
    {
        for i in hs.iter() {
            keys_as_found.push(i);
        }
        let mut z = keys_as_found.clone();
        z.sort();
        
        assert_eq!("bar", z[0]);
        assert_eq!("baz", z[1]);
        assert_eq!("foo", z[2]);
    }
    // Now test wrap
    {
        for i in hs.iter() {
            assert_eq!(keys_as_found[0], i);
            break;
        }
    }
    {
        for i in hs.iter() {
            assert_eq!(keys_as_found[1], i);
            break;
        }
    }
    {
        for i in hs.iter() {
            assert_eq!(keys_as_found[2], i);
            break;
        }
    }
    {
        for i in hs.iter() {
            panic!("We should have gotten NONE, instead we got {:?}", i);
        }
    }
    {
        for i in hs.iter() {
            assert_eq!(keys_as_found[0], i);
            break;
        }
    }
    hs.remove(&keys_as_found[1]);
    {
        let mut j = 0;
        for i in hs.iter() {
            assert_ne!(keys_as_found[1], i);
            j = j + 1;
        }
        assert_eq!(1, j);
    }
}
