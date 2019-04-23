//Loops
pub fn run() {
    let mut count = 0;

    //Infinite loop
    loop {
        count += 1;
        println!("NUmber : {}", count);
        if count == 10 {
            break;
        }
    }

    //While loop (fizzbuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("Fizzbuzz")
        } else if count % 3 == 0 {
            println!("Fizz")
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count)
        }
        count += 1;
    }

    //For loop
    for count in 1..100 {
        if count % 15 == 0 {
            println!("Fizzbuzz")
        } else if count % 3 == 0 {
            println!("Fizz")
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count)
        }
    }
}
