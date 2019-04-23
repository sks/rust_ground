use std::mem;

//vectors are resizable arrays
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("First value : {}", numbers[0]);

    println!("value after mutating : {:?}", numbers);
    //Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    //Re-assign value
    numbers[2] = 10;
    println!("{:?}", numbers);
    //Add value to vector
    numbers.push(100);
    println!("value after pushing : {:?}", numbers);

    numbers.pop();
    println!("value after popping : {:?}", numbers);

    //Length
    println!("Vector length {}", numbers.len());

    //Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Iterate over the number
    for x in numbers.iter() {
        println!("Number : {}", x);
    }
    //Mutate and iterate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("value after mutating : {:?}", numbers);
}
