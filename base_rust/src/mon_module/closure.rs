pub fn hello() {
    let a = 8;
    let b = 32;
    compute(add, a, b);
    compute(mulitply, a, b);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn mulitply(x: i32, y: i32) -> i32 {
    x * y
}

fn compute<F>(function: F, a: i32, b: i32)
where
    F: Fn(i32, i32) -> i32,
{
    println!("{}", function(a, b));
}
