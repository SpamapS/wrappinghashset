extern crate wrappinghashset;
use wrappinghashset::WrappingHashSet;

fn main() {
    let hs: WrappingHashSet<&str> = WrappingHashSet::new();
    println!("Yay {:?}", hs);
}
