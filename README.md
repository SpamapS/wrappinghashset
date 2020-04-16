Greetings! This is a tiny library I created to allow round-robin access to a
hashset. It stores an extra offset in the struct to keep track of which offset
it returned last time, so that the next call to iter() will return the next
item, and then only all of the rest, one time.

Please do report issues if you find them. Apologies for the lack of docs, but
it pretty much works like `std::collections::HashSet`
