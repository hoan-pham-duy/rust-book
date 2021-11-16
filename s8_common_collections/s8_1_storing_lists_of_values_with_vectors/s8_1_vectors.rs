

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    {
        let v4 = vec![1, 2, 3, 4];

        // do stuff with v4
    } // <- v4 goes out of scope and is freed here


    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    {

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        println!("v = {:?}", v);
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
    

}
