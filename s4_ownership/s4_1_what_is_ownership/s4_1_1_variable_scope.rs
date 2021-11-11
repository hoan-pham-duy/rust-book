fn main(){
    let s = "hello Scope Main function";
    {                      // s is not valid here, it’s not yet declared
        let s = "hello from smaller scope";   // s is valid from this point forward
        println!("From smaller scope, s = {}", s);
    }
    println!("In main function scope, s = {}", s);

}