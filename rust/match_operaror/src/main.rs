fn main() {
    let num = 1;


    match num {
        1 => println!("number 1"),
        2 => println!("number 2"),
        _ => println!("number not 1 or 2"), // equivalent to default??
    }

    let another_num = 8;


    match another_num {
        1 => println!("number 1"),
        2 => println!("number 2"),
        _ => println!("number not 1 or 2"), // equivalent to default??
    }


    let yet_another_num = 11;


    match yet_another_num {
        1 => println!("number 1"),
        10|11 => println!("number is 10 or 11"),
        _ => println!("number not 1 , 10, 11"), // equivalent to default??
    }

}
