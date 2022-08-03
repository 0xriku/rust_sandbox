// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Primitive str
    let _hello = "Hello";

    // String
    let mut greeting = String::from("Greetings ");

    // Get length
    println!("Length: {}", greeting.len());

    // Push char
    greeting.push('W');

    // Push string
    greeting.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", greeting.capacity());

    // Check if empty
    println!("Is Empty: {}", greeting.is_empty());

    // Contains
    println!("Contains 'World': {}", greeting.contains("World"));

    // Replace
    println!("Replace: {}", greeting.replace("World", "Friend"));

    // Loop through string by whitespace and put on separate lines
    for token in greeting.split_whitespace() {
        println!("{}", token);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", greeting);
}