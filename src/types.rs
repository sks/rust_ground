/*
Integers: u8, i8, ....
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {
    // Default is i32
    let x = -1;

    // Default f64
    let y = 2.5;

    let z: i64 = 234567890;

    let is_lessthan = 10 > 6;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!(
        "{:?}",
        (face, a1, std::i32::MAX, std::i64::MAX, is_lessthan)
    );
    println!("Max i64: {}", std::i64::MAX);
}
