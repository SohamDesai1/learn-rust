use std::f32::consts::PI;

struct Circle {
    radius: f32,
    diameter: f32,
}

impl Circle {
    fn circumference(&self) -> f32 {
        self.diameter * PI
    }

    fn area(&self) -> f32 {
        self.radius * self.radius * PI
    }

    fn eg() -> u32 {
        return 1;
    }
}

fn main() {
    let circle = Circle {
        radius: 16.0,
        diameter: 32.0,
    };

    println!(
        "The circumference of the circle is {}",
        circle.circumference()
    );
    println!("The area of the circle is {}", circle.area());
    println!("This is like a static fn which prints {}", Circle::eg());
}
