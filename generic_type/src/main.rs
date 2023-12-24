fn main() {
    let point1 = Point {
        x: 10.4,
        y: 10.5
    };
    println!("point1: ({}, {}), len = {}", point1.x(), point1.y(), point1.len());

    let person = Person {
        name: String::from("Wei Lei"),
        age: 32,
    };
    println!("{}", person.go());
    println!("{}", longest("I am short", "I am long long"));
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
    fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Person {
    name: String,
    age: i32,
}

impl Movable for Person {
    fn go(&self) -> String {
        return format!("Yes, {}<{}> let's go...", self.name, self.age);
    }
}

trait Movable {
    fn go(&self) -> String;
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> & 'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
