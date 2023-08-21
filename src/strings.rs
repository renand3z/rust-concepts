pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Empty?
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "Nigga"));

    // Loop
    for o in hello.split_whitespace() {
        println!("{}", o);
    }

    // Create string with capacity
    let mut s = String::with_capacity(0);
    println!("{}", s.capacity());
    s.push('x');
    s.push('y');

    println!("{}", s.capacity());
    println!("{}", s.capacity());

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(8, s.capacity());

    println!("{}", s);
} 