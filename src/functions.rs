pub fn run() {
    greeting("Hello there", "Sabith");
    println!("Sum of {0} and {1} is {2}", 1, 2, add(1, 2));

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| {
        let a = n1 * 100;
        a + n2 + n3
    };
    println!("C Sum: {}", add_nums(1, 2))
}

fn greeting(greet: &str, name: &str) {
    println!("Hi {} , {}", name, greet);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}
