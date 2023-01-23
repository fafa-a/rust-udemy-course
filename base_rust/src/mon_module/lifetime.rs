pub fn hello() {
    let a = 8;
    let b = 32;
    println!("{}", greather_than(&a, &b));
}

fn greather_than<'a>(a: &'a u8, b: &'a u8) -> &'a u8 {
    if a > b {
        a
    } else {
        b
    }
}
