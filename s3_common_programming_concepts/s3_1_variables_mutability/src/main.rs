const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Global scope
fn main() {
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    /* Difference with mutable
        // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
        // we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name
    */
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
    /*
    Compile errors
    let mut spaces = "   ";
    spaces = spaces.len();
    */
}