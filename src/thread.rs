use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn create_new_thread() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}

pub fn move_prop(){
    let vec = vec![1,2];

    let handle = thread::spawn( move || {
        println!("there is vector {:?}", vec);
    });
    handle.join().unwrap();
}

pub fn create_channel() {
    let (tx, rx) = mpsc::channel();
    let tx_copy = tx.clone();

    thread::spawn( move || {
        let vals = [
                String::from("Hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread")
            ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn( move || {
        let vals = [
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx_copy.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got 1: {}", received);
    }
    println!("new code");
}