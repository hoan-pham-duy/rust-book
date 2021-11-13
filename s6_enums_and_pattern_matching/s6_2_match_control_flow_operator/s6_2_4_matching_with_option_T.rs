fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("plus_one(five) = {:?}", plus_one(five));
    println!("plus_one(None) = {:?}", plus_one(None));
}
