struct Rectangle<T, U> {
    x: T,
    y: U,
}

pub fn hello() {
    let r1 = Rectangle { x: 10, y: 20 };
    let r2 = Rectangle { x: 2.0, y: 5 };
}
