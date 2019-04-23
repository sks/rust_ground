use std::fmt;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Color {{ red: {}, green: {}, blue: {} }}",
            self.red, self.green, self.blue
        )
    }
}

struct Point(u8, u8);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {}}}", self.0, self.1)
    }
}

struct Person {
    first_name: String,
    last_name: String,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Person")
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .finish()
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person [ {}]", self.full_name())
    }
}

impl Person {
    //Construct Person
    fn new(first: &str, last: &str) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        };
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    let c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("{:?}", c);

    let p = Point(10, 10);
    println!("{:?}", p);

    let mut person = Person::new("Sabith", "Soopy");
    println!("{}", person);
    person.last_name = "something".to_string();
    println!("{:?}", person);
}
