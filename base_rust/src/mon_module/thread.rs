use std::thread;
use std::time::Duration;

pub fn hello() {
    let th = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread {}", i);
            thread::sleep(Duration::from_millis(1000))
        }
    });
    for i in 1..5 {
        println!("Hello from the main thread {}", i);
        thread::sleep(Duration::from_millis(600))
    }
    th.join();
}
