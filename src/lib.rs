use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;
use std::marker::PhantomData;

/// A hash set that remembers the last key it returned with its iterator
/// it will wrap around and only return all of the keys once.
///
#[derive(Debug)]
pub struct WrappingHashSet<'a, T:'a>
    where T:Eq + Hash {
    hashset: HashSet<T>,
    keys: Vec<T>,
    pos: usize,
    count: usize,
    phantom: PhantomData<&'a T>,
}

pub struct Iter<'i, 'a: 'i, T:'a>
    where T:Eq + Hash {
    whs: &'i mut WrappingHashSet<'a, T>,
}

impl <'i, 'a, T:'a>Iterator for Iter<'i, 'a, T>
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

impl <'a, T:'a>WrappingHashSet<'a, T>
    where T:Eq + Hash + Clone {
    pub fn new() -> WrappingHashSet<'a, T> {
        WrappingHashSet {
            hashset: HashSet::new(),
            keys: Vec::new(),
            pos: 0,
            count: 0,
            phantom: PhantomData,
        }
    }

    pub fn iter<'i>(&'i mut self) -> Iter<'i, 'a, T> {
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

    pub fn remove(&mut self, key: &'a T) -> bool {
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
}
