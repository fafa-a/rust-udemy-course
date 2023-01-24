use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn hello() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..10 {
            sender.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in receiver {
        println!("Received: {}", received);
    }

    println!("End");
}
