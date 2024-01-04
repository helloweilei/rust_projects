use std::ops::Deref;

#[derive(Debug)]
pub enum List {
  Cons(i32, Option<Box<List>>),
  Nil
}

pub struct MySmartPointer {
  value: i32
}

impl MySmartPointer {
  pub fn new(value: i32) -> MySmartPointer {
    MySmartPointer{ value }
  }
}

impl Deref for MySmartPointer {
    type Target = i32;

    fn deref(&self) -> &i32 {
      &self.value
    }
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("drop MySmartPointer with value: {}", self.value);
    }
}

