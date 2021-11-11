fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s after changed = {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
