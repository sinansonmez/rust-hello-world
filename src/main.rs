// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

struct Point {
    x: u32,
    y: u32
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rect: Rectangle) -> u32 {
    let result = rect.top_left.y * rect.bottom_right.x;
    result
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0, y: 5 },
        bottom_right: Point { x: 10, y: 5 }
    };
    println!("-----------");
    println!("area is {:?}", rect_area(rect));
    println!("-----------");
}
