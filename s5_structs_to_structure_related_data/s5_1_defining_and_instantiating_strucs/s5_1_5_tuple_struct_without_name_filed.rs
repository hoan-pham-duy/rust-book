struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let color_all_1 = Color(1, 1, 1);
    // let color_all_0 = minus_1_color((1, 1, 1)); //error
    // let color_all_0 = minus_1_color(Color(1, 1, 1));
    println!("color_all_0 = {}, {}, {}", color_all_0.0, color_all_0.1, color_all_0.2);
}


fn minus_1_color(color: Color) -> Color {
    Color(color.0 - 1, color.1 - 1, color.2 - 1)
}