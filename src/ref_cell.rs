use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons,Nil};

#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

pub fn create_ref_cell() {
    let value = Rc::new(RefCell::new(21));

    let a = Rc::new(
        Cons(Rc::clone(&value),
             Rc::new(Nil))
    );
    let b = Cons(
        Rc::new(RefCell::new(33)),
        Rc::clone(&a)
    );
    let c = Cons(
        Rc::new(RefCell::new(23)),
        Rc::clone(&a)
    );

    *value.borrow_mut() += 100;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}