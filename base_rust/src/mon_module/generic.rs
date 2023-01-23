struct Rectangle<T> {
    x: T,
    y: T,
}

impl<T> Rectangle<T> {
    fn valeur(&self) -> &T {
        &self.x
    }
    fn surface(&self) -> &T {
        &self.x * &self.y
    }
}

pub fn hello() {
    let r1 = Rectangle { x: 10, y: 20 };
    let r2 = Rectangle { x: 2.7, y: 5.9 };
    println!("r1.x = {}", r2.surface());
}
