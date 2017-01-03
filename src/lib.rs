use std::iter::{Cycle, Take};
use std::collections::HashSet;
use std::collections::hash_set::Iter;

/// A hash set that remembers the last key it returned with its iterator
/// it will wrap around and only return all of the keys once.
///
struct WrappingHashSet<'a> {
    hashset: HashSet<&'a str>,
    cycle: Option<Cycle<Iter<'a, &'a str>>>,
}

impl<'a>  WrappingHashSet<'a> {
    fn new() -> WrappingHashSet<'a> {
        let mut w = WrappingHashSet {
            hashset: HashSet::new(),
            cycle: None,
        };
        w.cycle = Some(w.hashset.iter().cycle());
        w

    }
    fn iter(&mut self) -> Take<Cycle<Iter<&'a str>>> {
        self.cycle.unwrap().take(self.hashset.len())
    }
    fn insert(&mut self, key: &'a str) -> bool {
        self.hashset.insert(key)
    }
    fn remove(&mut self, key: &'a str) -> bool {
        self.hashset.remove(key)
    }
}

#[test]
fn test_wrapping_hashset() {
    let mut hs = WrappingHashSet::new();
    hs.insert("foo");
    hs.insert("bar");
    hs.insert("baz");
    {
        let mut x = hs.iter();
        let mut z = Vec::new();
        for i in x {
            z.push(*i);
        }
        z.sort();
        
        assert_eq!("bar", z[0]);
        assert_eq!("baz", z[1]);
        assert_eq!("foo", z[2]);
    }
    // Now test wrap
    for i in hs.iter() {
        assert_eq!("bar", *i);
        break;
    }
    for i in hs.iter() {
        assert_eq!("baz", *i);
        break;
    }
}
