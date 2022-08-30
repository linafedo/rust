use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn create_mutex() {
    let counter = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();
            num.push_str(String::from(format!("\n {i} + 🥰")).as_str());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", counter.lock().unwrap());
}