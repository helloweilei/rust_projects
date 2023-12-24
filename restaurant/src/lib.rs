pub mod log;

mod math {
    pub mod operation {
        pub fn add(left: i32, right: i32) -> i32 {
            left + right
        }

        pub fn minus(left: i32, right: i32) -> i32 {
            left - right
        }

        pub const PI: f64 = 3.1415926;

        pub enum Compare {
            Greater,
            Equal,
            Less,
        }
        pub struct Complex {
            pub real: i32,
            pub img: i32,
        }

        impl Complex {
            pub fn new(real: i32, img: i32) -> Complex {
                Complex {
                    real: real,
                    img: img,
                }
            }
        }
    }
}

pub fn test_method() {
    math::operation::add(2, 4); // 相对路径
    crate::math::operation::minus(10, 4); // 绝对路径
    println!("{}", math::operation::PI);
}

use math::operation::Compare;

pub fn compare(a: i32, b: i32) -> Compare {
    if a > b {
        Compare::Greater
    } else if a < b {
        Compare::Less
    } else {
        Compare::Equal
    }
}

use math::operation::Complex;
pub fn print_complex() {
    let complex: Complex = Complex::new(3, 4);
    println!("{} + i{}", complex.real, complex.img);
}
