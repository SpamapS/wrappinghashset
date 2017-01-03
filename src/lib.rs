use std::iter::Iterator;
use std::collections::HashSet;

/// A hash set that remembers the last key it returned with its iterator
/// it will wrap around and only return all of the keys once.
///
#[derive(Debug)]
pub struct WrappingHashSet<'a> {
    hashset: HashSet<&'a str>,
    keys: Vec<&'a str>,
    pos: usize,
    count: usize,
}

pub struct Iter<'i, 'a: 'i> {
    whs: &'i mut WrappingHashSet<'a>,
}

impl <'i, 'a>Iterator for Iter<'i, 'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
       self.whs.pos += 1;
       self.whs.count += 1;
       if self.whs.count > self.whs.hashset.len() {
           self.whs.pos = 0;
           self.whs.count = 0;
           return None;
       }
       Some(self.whs.keys[self.whs.pos - 1])
    }
}

impl <'a>WrappingHashSet<'a> {
    pub fn new() -> WrappingHashSet<'a> {
        WrappingHashSet {
            hashset: HashSet::new(),
            keys: Vec::new(),
            pos: 0,
            count: 0,
        }
    }

    pub fn iter<'i>(&'i mut self) -> Iter<'i, 'a> {
        Iter {
            whs: self,
        }
    }

    pub fn insert(&mut self, key: &'a str) -> bool {
        if self.hashset.insert(key) {
            self.keys.push(key);
            return true
        }
        return false
    }

    pub fn remove(&mut self, key: &'a str) -> bool {
        if self.hashset.remove(key) {
            self.keys = Vec::new();
            for key in self.hashset.iter() {
                self.keys.push(key)
            }
            return true
        }
        return false
    }
}

#[test]
fn test_wrapping_hashset() {
    let mut hs = WrappingHashSet::new();
    let mut keys_as_found = Vec::new();
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
            panic!("We should have gotten NONE");
            break;
        }
    }
    {
        for i in hs.iter() {
            assert_eq!(keys_as_found[0], i);
            break;
        }
    }
}
