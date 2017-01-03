use std::iter::Iterator;
use std::collections::HashSet;

/// A hash set that remembers the last key it returned with its iterator
/// it will wrap around and only return all of the keys once.
///
struct WrappingHashSet<'a> {
    hashset: HashSet<&'a str>,
    keys: Vec<&'a str>,
    pos: usize,
    count: usize,
}

struct Iter<'i, 'a: 'i> {
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
    fn new() -> WrappingHashSet<'a> {
        WrappingHashSet {
            hashset: HashSet::new(),
            keys: Vec::new(),
            pos: 0,
            count: 0,
        }
    }

    fn iter<'i>(&'a mut self) -> Iter<'i, 'a> {
        Iter {
            whs: self,
        }
    }

    fn insert(&mut self, key: &'a str) -> bool {
        if self.hashset.insert(key) {
            self.keys.push(key);
            return true
        }
        return false
    }

    fn remove(&mut self, key: &'a str) -> bool {
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
    hs.insert("foo");
    hs.insert("bar");
    hs.insert("baz");
    {
        let mut z = Vec::new();
        for i in hs.iter() {
            z.push(i);
        }
        z.sort();
        
        assert_eq!("bar", z[0]);
        assert_eq!("baz", z[1]);
        assert_eq!("foo", z[2]);
    }
    // Now test wrap
    {
        for i in hs.iter() {
            assert_eq!("bar", i);
            break;
        }
    }
    {
        for i in hs.iter() {
            assert_eq!("baz", i);
            break;
        }
    }
}
