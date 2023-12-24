struct User {
    first_name: String,
    last_name: String,
    age: u16,
    birth: (u16, u8, u8),
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数
    fn from(size: (u32, u32)) -> Rect {
        Rect {
            width: size.0,
            height: size.1,
        }
    }
}

fn show_user(user: &User) {
    println!("user info: first_name => {}, last_name => {}, age => {}, birth => {}",
        &user.first_name,
        &user.last_name,
        user.age,
        user.birth.0
    );
}

fn main() {
    let user = User {
        first_name: String::from("wei"),
        last_name: String::from("lei"),
        age: 32,
        birth: (1991, 10, 10),
    };
    show_user(&user);
    let rect = Rect {
        width: 30,
        height: 40,
    };
    println!("rect detail is: {:?}", rect);
    println!("the area is: {}", rect.area());

    let rect2 = Rect::from((20, 20));
    println!("rect can hold rect1 ? {}", rect.can_hold(&rect2));
}
