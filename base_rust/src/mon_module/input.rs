use std::fs::OpenOptions;
use std::io::{stdin, Write};

pub fn hello() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("test.txt")
        .unwrap();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    file.write(s.as_bytes()).unwrap();
}
