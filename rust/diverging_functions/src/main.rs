fn main() {
    
    diverging1();
}

fn diverging1 () -> !{
    panic!("This function never returns!");
}