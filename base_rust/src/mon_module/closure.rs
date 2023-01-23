pub fn hello() {
    let a = 8;
    let b = 32;
    // compute(|x, y| x + y, a, b);
    compute(|| a + b);
}

// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
//
// fn mulitply(x: i32, y: i32) -> i32 {
//     x * y
// }

// fn compute<F>(function: F, a: i32, b: i32)
fn compute<F>(function: F)
where
    // F: Fn(i32, i32) -> i32,
    F: Fn() -> i32,
{
    println!("{}", function());
}
