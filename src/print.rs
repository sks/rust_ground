pub fn run() {
    //Print to console
    println!("Hello from print.rs");
    println!("{} is from {}", "Sabith", "Kerala");
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Sabith", "Kerala", "Code",
    );
    println!(
        "{name} likes to {activity}",
        name = "Sabith",
        activity = "footbal"
    );
    println!("Binary: {:b}, Hex {:x}, OCtal: {:o}", 10, 10, 10);
}
