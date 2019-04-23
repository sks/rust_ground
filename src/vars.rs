pub fn run() {
    let name = "Sabith";
    println!("My name is {}", name);
    let mut age = 30;
    println!("My name is {} and is {} years old", name, age);
    age = 38;

    const ID: i32 = 1;
    println!("My name is {} and is {} years old, {}", name, age, ID);
    let (my_name, my_age) = ("SKS", 30);
    println!("My name is {} and is {} years old, {}", my_name, my_age, ID);
}
