use std::cell::RefCell;
use std::rc::Rc;

type NodePointer<T> = Option<Rc<RefCell<T>>>;

#[derive(Debug)]
struct Node<T: std::fmt::Display + Copy> {
  value: T,
  prev: NodePointer<Node<T>>,
  next: NodePointer<Node<T>>
}

impl<T: std::fmt::Display + Copy> Node<T> {
  fn new(value: T) -> Node<T> {
    Node { value, prev: None, next: None }
  }
}

#[derive(Debug)]
pub struct LinkedList<T: std::fmt::Display + Copy> {
  head: NodePointer<Node<T>>,
  tail: NodePointer<Node<T>>,
}

impl<T: std::fmt::Display + Copy> LinkedList<T> {
  pub fn new() -> LinkedList<T> {
    LinkedList{ head: None, tail: None }
  }

  pub fn push_front(&mut self, value: T) {
    let new_head = Rc::new(RefCell::new(Node::new(value)));
    match self.head.take() {
        Some(old_head) => {
          (*new_head).borrow_mut().next = Some(old_head.clone());
          (*old_head).borrow_mut().prev = Some(new_head.clone());
          self.head = Some(new_head.clone());
        },
        None => {
          self.head = Some(new_head.clone());
          self.tail = Some(new_head.clone());
        }
    }
  }

  pub fn push_back(&mut self, value: T) {
    let new_tail = Rc::new(RefCell::new(Node::new(value)));
    match self.tail.take() {
      Some(old_tail) => {
        (*old_tail).borrow_mut().next = Some(new_tail.clone());
        (*new_tail).borrow_mut().prev = Some(old_tail.clone());
        self.tail = Some(new_tail.clone());
      },
      None => {
        self.tail = Some(new_tail.clone());
        self.head = Some(new_tail.clone());
      }
    }
  }

  pub fn remove_front(&mut self) -> Option<T> {
    let old_head = self.head.take();
    if old_head.is_none() {
      print!("list is empty!");
      None
    } else {
      old_head.map(|head_node| {
        match (*head_node).borrow_mut().next.take() {
          Some(new_head) => {
            self.head = Some(new_head);
          },
          None => {
            self.head = None;
            self.tail = None;
          }
        }
        (*head_node).borrow().value
      })
    }
  }

  pub fn list_all(&self) -> Vec<T> {
    let mut values = vec![];
    let mut current = self.head.clone();
    loop {
        match current {
            Some(cur_node) => {
              values.insert(values.len(), (*cur_node).borrow().value);
              current = (*cur_node).borrow().next.clone();
            },
            None => { break }
        }
    }
    values
  }
}