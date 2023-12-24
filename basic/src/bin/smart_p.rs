use basic::smart_pointer::List;

fn main() {
  let list = List::Cons(10, Box::new(List::Cons(20, Box::new(List::Nil))));

  println!("list is: {:?}", list);

  if let List::Cons(_, second) = list {
    if let List::Cons(value, _) = *second {
      println!("the second value is: {}", value);
    }
  }
}