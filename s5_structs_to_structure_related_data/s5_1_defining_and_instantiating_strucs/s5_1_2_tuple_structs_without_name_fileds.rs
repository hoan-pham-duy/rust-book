fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(5, 6, 7);
    let origin = Point(0, 0, 0);

    println!("black = {}", black.0);
}
