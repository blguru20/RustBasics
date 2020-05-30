
mod guru {

    fn print_guru(){
        println!("I am Guru! :) ");
    }

   pub fn print_nayana () {
        println!("I am Nayana,  I can call Guru :-  :) ");
        print_guru();
    }



    pub mod water {
        pub fn print_msg(){
            println!("water! ");
        }
    }

}


fn main() {
 //    guru::PrintGuru(); causes error as it is not pub, private 
 guru::print_nayana();

 guru::water::print_msg();
}
