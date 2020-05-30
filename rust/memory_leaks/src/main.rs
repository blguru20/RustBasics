fn main() {
    use std::mem;
    use std::fs::File;
    
    let file = File::open("foo.txt").unwrap();
    mem::forget(file);
}
