use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn hello() {
    let (sender, receiver) = mpsc::channel();
    let sender2 = mpsc::Sender::clone(&sender);

    thread::spawn(move || {
        for i in 1..10 {
            sender.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        for i in 10..20 {
            sender2.send(i).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in receiver {
        println!("Received: {}", received);
    }

    println!("End");
}
