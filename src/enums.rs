enum Movement {
    Up,
    Down,
    Left,
    Right,
}
fn move_avatar(m: Movement) {
    //Perform action depending on m
    match m {
        Movement::Up => println!("Move up"),
        Movement::Down => println!("Move Down"),
        Movement::Right => println!("Move Right"),
        Movement::Left => println!("Move Left"),
    }
}
pub fn run() {
    move_avatar(Movement::Left);
    move_avatar(Movement::Right);
    move_avatar(Movement::Up);
    move_avatar(Movement::Down);
}
