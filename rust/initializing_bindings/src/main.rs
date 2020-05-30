fn main() {
    let x: i32 = 10;
    let y : i32;
    println!("The value of x is: {}", x);
 //    println!("The value of y is: {}", y); //error[E0381]: borrow of possibly-uninitialized variable: `x`
                                             //  ^ use of possibly-uninitialized `y`
}


fn demo_scope_shadows(){

    #![allow(unused_variables)]
    fn main() {
    let mut x: i32 = 1;
    x = 7;
    let x = x; // `x` is now immutable and is bound to `7`.
    
    let y = 4;
    let y = "I can also be bound to text!"; // `y` is now of a different type.
    }
}