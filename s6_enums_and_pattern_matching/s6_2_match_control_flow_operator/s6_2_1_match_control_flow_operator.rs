enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter;
    let val_of_pennny = value_in_cents(penny);
    println!("val_of_pennny = {}", val_of_pennny);

    println!("val_of_quarter = {}", value_in_cents(Coin::Quarter));
}
