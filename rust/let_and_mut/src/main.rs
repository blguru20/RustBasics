fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    let _x = 5; // using x instead of x creates unused variable warning
  //   x = 10; // error[E0384]: cannot assign twice to immutable variable `x` // hence commented. 
    let mut _y = 10.01010;
   //  y = 100; //error[E0308]: mismatched types // expecting floating point integer 
   
   _y  = 1.01;
   println!("{} ", _y);


let r;              // Introduce reference: `r`.
{
    let i = 1;      // Introduce scoped value: `i`.
  
    //r = &i;         // Store reference of `i` in `r`.
}                   // `i` goes out of scope and is dropped.

// println!("{}", r);  // `r` still refers to `i`. // E0597 //borrowed value does not live long enough


}
