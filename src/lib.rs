/*
 * Copyright 2017, 2024 Clint Byrum
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
/// it will wrap around and only return all of the keys once per iterator
///
/// Important: Prior to version 0.5 all iterators shared the count, so
/// if you only partilaly read one iterator, it would affect the next one.
/// As of 0.5 and forward, the position is remembered, but the count is
/// forgotten. This may break your app if you are depending on the old
/// behavior.
#[derive(Debug)]
pub struct WrappingHashSet<T>
where
    T: Eq + Hash,
{
    hashset: HashSet<T>,
    keys: Vec<T>,
    pos: usize,
}

pub struct Iter<'i, T: 'i>
where
    T: Eq + Hash,
{
    whs: &'i mut WrappingHashSet<T>,
    count: usize,
}

impl<'i, T> Iterator for Iter<'i, T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // Wrap
        if self.whs.pos >= self.whs.hashset.len() {
            self.whs.pos = 0;
        }
        self.count += 1;
        if self.count > self.whs.hashset.len() {
            self.count = 0;
            return None;
        }
        self.whs.pos += 1;
        Some(self.whs.keys[self.whs.pos - 1].clone())
    }
}

impl<T> WrappingHashSet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> WrappingHashSet<T> {
        WrappingHashSet {
            hashset: HashSet::new(),
            keys: Vec::new(),
            pos: 0,
        }
    }

    pub fn iter<'i>(&'i mut self) -> Iter<'i, T> {
        Iter {
            whs: self,
            count: 0,
        }
    }

    pub fn insert(&mut self, key: T) -> bool {
        if self.hashset.insert(key.clone()) {
            self.keys.push(key);
            return true;
        }
        return false;
    }

    pub fn remove<'b>(&mut self, key: &'b T) -> bool {
        if self.hashset.remove(key) {
            self.keys = Vec::new();
            for k in self.hashset.iter() {
                self.keys.push(k.clone())
            }
            return true;
        }
        return false;
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
        // It returned all 3
        assert_eq!("bar", z[0]);
        assert_eq!("baz", z[1]);
        assert_eq!("foo", z[2]);
    }
    // Now test wrap
    {
        for i in hs.iter() {
            assert_eq!(keys_as_found[0], i, "First Iter returns first element");
            break;
        }
    }
    {
        for i in hs.iter() {
            assert_eq!(
                keys_as_found[1], i,
                "Second Iter returns second element first"
            );
            break;
        }
    }
    {
        for i in hs.iter() {
            assert_eq!(
                keys_as_found[2], i,
                "Third Iter returns third element first"
            );
            break;
        }
    }
    // Now it should wrap because we have a new iterator
    {
        for i in hs.iter() {
            assert_eq!(
                keys_as_found[0], i,
                "Fourth Iter returns first element first"
            );
            break;
        }
    }
    {
        let mut iter = hs.iter();
        assert_eq!(Some(keys_as_found[1]), iter.next());
        assert_eq!(Some(keys_as_found[2]), iter.next());
        assert_eq!(Some(keys_as_found[0]), iter.next());
        assert_eq!(None, iter.next(), "Should wrap only once");
    }
    {
        let mut iter = hs.iter();
        assert_eq!(Some(keys_as_found[1]), iter.next());
        assert_eq!(Some(keys_as_found[2]), iter.next());
        assert_eq!(Some(keys_as_found[0]), iter.next());
        assert_eq!(None, iter.next(), "Should repeat");
    }
    // Now use it partially
    {
        let mut iter = hs.iter();
        assert_eq!(Some(keys_as_found[1]), iter.next());
    }
    // Still picks up where it was
    {
        let mut iter = hs.iter();
        assert_eq!(Some(keys_as_found[2]), iter.next());
    }
    hs.remove(&keys_as_found[1]);
    {
        let mut j = 0;
        for i in hs.iter() {
            assert_ne!(keys_as_found[1], i, "Elements should not reappear");
            j = j + 1;
        }
        assert_eq!(2, j, "We should only iterate the leftover elements");
    }
}

#[test]
fn test_empty() {
    let mut hs: WrappingHashSet<&str> = WrappingHashSet::new();
    {
        let mut hsiter = hs.iter();
        assert_eq!(None, hsiter.next());
    }
    hs.insert("one");
    hs.insert("two");
    let mut hsiter = hs.iter();
    assert_eq!(Some("one"), hsiter.next());
    assert_eq!(Some("two"), hsiter.next());
    assert_eq!(None, hsiter.next());
}

#[test]
fn test_one_item() {
    let mut hs: WrappingHashSet<&str> = WrappingHashSet::new();
    hs.insert("onething");
    {
        let mut hsiter = hs.iter();
        assert_eq!(Some("onething"), hsiter.next());
        assert_eq!(None, hsiter.next());
    }
    {
        let mut _hsunused = hs.iter();
        // We never call this so pos should stay where it was
    }
    {
        let mut hsiter = hs.iter();
        assert_eq!(Some("onething"), hsiter.next());
        assert_eq!(None, hsiter.next());
    }
}
