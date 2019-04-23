pub fn run() {
    let mut hello = String::from("Hello");
    hello.push(' ');
    hello.push_str("World");
    println!("Length: {}", hello.len());
    println!("Capacity: {}", hello.capacity());
    println!("{}", hello);
    for word in hello.split_whitespace() {
        println!("Split word : {}", word)
    }
    let mut word_with_capacity = String::with_capacity(10);
    word_with_capacity.push('A');
    word_with_capacity.push('B');
    assert_eq!(10, word_with_capacity.capacity());
    assert_eq!(2, word_with_capacity.len());
    if word_with_capacity == "AB" {
        println!("we have the test")
    }
    println!("{}", word_with_capacity);
}
