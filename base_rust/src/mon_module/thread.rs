use std::sync::{Arc, Mutex};
use std::thread;

pub fn hello() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let count = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Hello from the main thread! {}", *counter.lock().unwrap());
}
