trait Compute {
    fn area(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Compute for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

struct Circle {
    radius: f32,
}

impl Compute for Circle {
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
    computed(r)
    computed(c);
}
// fn computed<T: Compute>(figure: T) {
//    println!("{}", figure.area());
//    }
fn computed(figure: impl Compute) {
    println!("{}", figure.area());
}
