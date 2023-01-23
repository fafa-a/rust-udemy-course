struct Rectangle<T> {
    x: T,
    y: T,
}

impl Rectangle<usize> {
    fn surface(&self) -> usize {
        &self.x * &self.y
    }
}
impl Rectangle<f32> {
    fn surface(&self) -> f32 {
        &self.x * &self.y
    }
}

pub fn hello() {
    let r1 = Rectangle { x: 10, y: 20 };
    let r2 = Rectangle { x: 2.7, y: 5.9 };
    println!("r1 surface = {}", r1.surface());
    println!("r2 surface = {}", r2.surface());
}
