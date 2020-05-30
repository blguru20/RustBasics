fn main() {

    let ret = add_one(1);
    let ret2 = add_two(2);
    println!("{} Hello, world!", ret);
    println!("{} Hello, world2!", ret2);
    return ();
}

fn add_one(x: i32 ) -> i32 {
   // x +1 ; // error[E0308]: mismatched types  // expecting i32 got ()
    x + 1   // notice no semicolons, doesnt require return keyword
}


fn add_two(x: i32 ) -> i32 {
    // x +1 ; // error[E0308]: mismatched types  // expecting i32 got ()
    return  x + 2 ;

    x+1  // warning unreachable expression
 }