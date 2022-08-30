use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons,Nil};

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

pub fn retain_cycle() {
    let a = Rc::new(
        Cons(12, RefCell::new(Rc::new(Nil)))
    );
    println!("initial ref count {:?}", Rc::strong_count(&a));
    println!("next tem {:?}", a.tail());

    let item = Rc::downgrade(&a).upgrade();
    if let Some(item) = item {
        let b = Rc::new(
            Cons(12, RefCell::new(item))
        );
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // println!("a next item = {:?}", a.tail());
    }
}