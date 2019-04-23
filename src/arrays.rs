//Arrays - fixed list where elements are same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("First value : {}", numbers[0]);
    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    //Re-assign value
    numbers[2] = 10;
    println!("{:?}", numbers);

    //Length
    println!("{}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get slices from array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("Slice occupies {} bytes", mem::size_of_val(&slice));
}
