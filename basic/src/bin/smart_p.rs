use basic::{smart_pointer::{List, MySmartPointer}, linked_list::{self, LinkedList}};

fn main() {
  // let list = List::Cons(10, Some(Box::new(List::Cons(20, None))));

  // println!("list is: {:?}", list);

  // if let List::Cons(_, second) = list {
  //   if let Some(value) = second {
  //     if let List::Cons(value, _) = *value {
  //       println!("the second value is: {}", value);
  //     }
  //   }
  // }

  // let a = 50;
  // let ptr = MySmartPointer::new(a);
  // let ptr2 = MySmartPointer::new(a + 10);

  // println!("a == *ptr: {}", a == *ptr);

  // drop(ptr2);
  let mut linked_list = LinkedList::new();
  linked_list.push_front(12);
  linked_list.push_front(11);
  linked_list.push_back(13);
  linked_list.push_back(14);
  linked_list.remove_front();
  linked_list.remove_front();

  print!("linked list: {:?}", linked_list.list_all());
}