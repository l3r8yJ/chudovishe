// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rectangle_area(rect: &Rectangle) -> f32 {
    ((rect.bottom_right.x - rect.top_left.x) * (rect.bottom_right.y - rect.top_left.y))
        .abs()
}

fn square(x: f32, y: f32, bottom_right: Point) -> (f32, f32) {
    ((bottom_right.x - x).abs(), (bottom_right.y - y).abs())
}

fn main() {
    let r = Rectangle {
        top_left: Point { x: 1.0, y: 10.0 },
        bottom_right: Point { x: 5.0, y: 1.0 },
    };
    println!("{:?} area is {}", r, rectangle_area(&r));
    let s = square(-3.0, 6.0, Point { x: 4.0, y: -2.0 });
    println!("h is {:.2}, w is {:.2}", s.0, s.1)
}
