struct ToSay<'a> {
    text: &'a str,
}
pub fn hello() {
    // let a = 8;
    // let b = 32;
    // println!("{}", greather_than(&a, &b));
    let a = ToSay { text: "Hello" };
    println!("{}", a.text);
}

fn greather_than<'a>(a: &'a u8, b: &'a u8) -> &'a u8 {
    if a > b {
        a
    } else {
        b
    }
}
