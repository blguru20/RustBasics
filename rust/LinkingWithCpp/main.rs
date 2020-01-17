// main.rs
extern {
    fn doubler(x: u8) -> u8;
}

fn main() {
    unsafe {
        println!("{}", doubler(1));
    }
}
