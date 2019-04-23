pub fn run() {
    let age = 21;
    let knows_person_of_age: bool = true;
    if age >= 21 || knows_person_of_age {
        println!("Give a beer");
    } else {
        println!("Give a candy");
    }

    //Short hand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is Of age : {}", is_of_age);
}
