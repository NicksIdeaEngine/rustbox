// primitive str = immutable fixed-length string somewhere in memory
// string = growable, heap-allocated data structure. use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // get length
    println!("length: {}", hello.len());
    // push char
    hello.push('W');
    // push string
    hello.push_str("orld!");
    // capacity in bytes
    println!("capacity: {}", hello.capacity());
    // check if empty
    println!("is empty: {}", hello.is_empty());
    // contains
    println!("contains 'world' {}", hello.contains("World"));
    // replace
    println!("replace: {}", hello.replace("World", "There"));
    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
