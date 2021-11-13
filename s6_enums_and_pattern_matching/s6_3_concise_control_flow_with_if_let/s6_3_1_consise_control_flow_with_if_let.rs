fn main() {
    let max = 9;
    let config_max = 3;
    if max == config_max {
        println!("The maximum is configured to be {:?}", config_max);
    } else {
        println!("The maximum is not configured to be {:?}", config_max);
    }
}
