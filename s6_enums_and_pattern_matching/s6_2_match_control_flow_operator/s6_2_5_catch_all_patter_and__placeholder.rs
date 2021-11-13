fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        _ => reroll(),//If we don't want
        /*other or _ will run before depend on the order*/
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("Inside other, num_spaces = {}", num_spaces);
    }
    fn reroll(){
        println!("Go to reroll");
    }
}
