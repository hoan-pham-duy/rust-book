use std::io;
fn main() {
    // Need expected data type when parse
    let guess: u32 = "42".parse().expect("Not a number!");
    
    //**********Scalar Types ****************************/
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    
    let t = true;

    let f: bool = false; // with explicit type annotation    

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

//**********Compound Types ****************************/    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    
    let a = [1, 2, 3, 4, 5];
    /*i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.*/
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    /*a will contain 5 elements that will all be set to the value 3 initially. */
    let a = [3; 5];

    let first = a[0];
    let second = a[1];    

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );    
}
