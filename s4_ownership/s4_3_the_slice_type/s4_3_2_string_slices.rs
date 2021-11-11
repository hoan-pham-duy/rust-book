fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

println!("&s[3..len] = {}", &s[3..len]);
println!("&s[1..len-1]; = {}", &s[1..len-1]);
}
