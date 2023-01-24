use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Write};
pub fn hello() {
    let path = "test.txt";

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let mut output = File::create(path).unwrap();
    output.write_all(s.to_uppercase().as_bytes()).unwrap();

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}
