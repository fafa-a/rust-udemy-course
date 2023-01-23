struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        &self.radius.powf(2.0) * std::f32::consts::PI
    }
}

pub fn hello() {
    let r = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let c = Circle { radius: 10.0 };
    println!("r area = {}", r.area());
    println!("c area = {}", c.area());
}

fn compute<T>(figure: T) {
    println!("{}", figure.area());
}
