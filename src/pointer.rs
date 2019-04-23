pub fn run() {
    //Primitives
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Value : {:?}", (arr1, arr2));

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Value : {:?}", (&vec1, vec2));
}
