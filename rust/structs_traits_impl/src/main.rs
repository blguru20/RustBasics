

mod somemod {
    pub struct Guru {
        pub  a : u8,
        pub  b : u8, 
        pub  c : u8,
    }
}



     struct Manu {
          a : u8,
          b : u8, 
          c : u8,
    }

    impl Manu {
        fn PrintValues(&self){
            println!("printing from impl function : {}", self.a);
        }
    }



fn main() {
    let  g = somemod::Guru{a : 8,b : 0,c : 9};
    let mut  m = Manu{a : 8,b : 0,c : 9};
    m.a = 100;



    println!("printing structs members from outside : g.a : {}", g.a);
    println!("printing structs members from outside : m.a : {}", m.a);

    m.PrintValues();

}

