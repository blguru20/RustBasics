

fn main() {
    let f: fn(i32) -> i32;
// Without type inference:
let f1: fn(i32) -> i32 = plus_one;

// With type inference:
let f2 = plus_one;   // todo :  redefinition doesnt cause issue in rust?

let ret = f2(5);






}


// let iu = 0; cannot declare variable outside a function?
fn plus_one(i: i32) -> i32 {
    i + 1
}

