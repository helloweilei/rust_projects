// fn main() {
//     let s1 = 10;
//     let _s2 = s1;
//     print!("value of s1: {}", s1); // OK

//     let mut my_car = Car{owner: "charlie".to_string(), year:2023, price: 5_000.0};
//     my_car.owner = String::from("John");

//     let another_car = Car{
//         year: 2022,
//         ..my_car
//     };
//     let owner = my_car.owner;
//     //print!("owner: {}", my_car.owner); // error: borrow of moved value: `my_car.owner`

//     let point = Point3D(1.0, 1.0, 0.0);

//     let new_car = Car::new("Petter".to_string(), 3_000.0);
//     print!("price: {}", new_car.current_price());
//     let sold_car = new_car.sell("Jenny".to_string());
// }

// struct Car {
//     owner: String,
//     year: i32,
//     price: f32,
// }

// impl Car {
//     fn current_price(&self) -> f32 {
//         self.price
//     }

//     // 不带&的self会将实例的所有权转移到方法内部
//     fn sell(self, new_owner: String) -> Self {
//         Car { owner: new_owner, year: self.year, price: self.price }
//     }

//     // 关联函数
//     fn new(owner: String, price: f32) -> Self {
//         Car { owner, year: 2023, price }
//     }
// }

// struct Point3D(f32, f32, f32);

use std::f32::consts::PI;

enum WeekDay {
    Monday,
    Tuesday,
    WednesDay,
    Thursday,
    Friday,
    SaturDay,
    Sunday
}

enum Shape {
    Circle(f32),
    Rectangle {
        width: f32,
        height: f32,
    },
    Square(f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Square(a) => a.powi(2),
        }
    }
}

#[derive(Debug)]
struct Person {

    pub first_name: String,

    pub last_name: Option<String>,

    pub age: i32,

}

fn first_char(char_vec: &Vec<char>) -> Option<char> {
    if char_vec.len() > 0 {
        Some(char_vec[0])
    } else {
        None
    }
}

// fn main() {
//     // let chars = vec!['c', 'a', 'z'];
//     // if let Some(c) = first_char(&chars) {
//     //     print!("First char: {c}");
//     // }
//     // let vec1 = vec![1,3,4,6];
//     // let vec2 = vec![3,4,7,9,10];
//     // let inter = basic::intersection(&vec1, &vec2);

//     // println!("intersection: {:?}", inter);
//     let mut persons: Vec<Person> = Vec::new();

//     persons.push(Person {

//         first_name: "Nouman".to_string(),

//         last_name: Some("Azam".to_string()),

//         age: 1,

//     });



//     persons.push(Person {

//         first_name: "Kamran".to_string(),

//         last_name: Some("Khan".to_string()),

//         age: 2,

//     });



//     persons.push(Person {

//         first_name: "Rahul".to_string(),

//         last_name: None,

//         age: 6,

//     });



//     persons.push(Person {

//         first_name: "Imran".to_string(),

//         last_name: Some("Rehman".to_string()),

//         age: 6,

//     });

//     println!("persons: {:?}", persons.iter().collect::<Vec<_>>())
// }

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}



fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };
    println!("{} by {}", book.title, book.author);

}