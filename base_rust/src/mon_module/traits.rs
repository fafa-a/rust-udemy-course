trait Area {
    fn area(&self) -> f32;
}

trait Perimeter {
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    width: f32,
    length: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.length
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f32 {
        2.0f32 * (self.width + self.length)
    }
}

// impl Rectangle {
//     fn hello(&self) {
//         println!("Hello");
//     }
// }

// struct Circle {
//     radius: f32,
// }
//
// impl Compute for Circle {
//     fn area(&self) -> f32 {
//         &self.radius.powf(2.0) * std::f32::consts::PI
//     }
// }

pub fn hello() {
    let r = Rectangle {
        width: 10.0,
        length: 20.0,
    };
    let r2 = Rectangle {
        width: 20.0,
        length: 30.0,
    };

    computed(r, r2);
}

// fn computed<T: Area + Perimeter>(figure: T) {
//    println!("{}", figure.area());
//    }

// fn computed(figure: impl Area, figure2: impl Perimeter) {
//fn computed<T: Area, U: Perimeter>(figure: T, figure2: U) {
fn computed<T, U>(figure: T, figure2: U)
where
    T: Area,
    U: Perimeter,
{
    println!("Area {}", figure.area());
    println!("Perimeter {}", figure2.area());
}
