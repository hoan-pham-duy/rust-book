fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}


/* Counting Loop:
    1st counting loop: count = 0, remaining = 10
        go to sub loop:
            remaining = 10, remaining -= 1 -> remaining = 9
            remaining = 9, break of subloop
        
        count +=1 ->  count = 1; Set up remaining = 10
            go to subloop:
                remaining = 10, remaining -= 1 -> remaining = 9
                remaining = 9, break of subloop       
        count += 1 -> count = 2; Set up remaining = 10
            go to subloop:
                remaining = 10
                count = 2 -> break lable: counting_up loop
    
    End Count



*/