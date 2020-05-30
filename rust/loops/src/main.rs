fn main() {


    println!("calling for loop ");
    let mut flag : bool = true;
    while(flag){
        flag = false;
        println!("inside while loop ");
    }
 println!("calling for loop ");
    for x in 0..10{
        println!("value : {}", x);
    }

        //infinite loop 
        let mut count = 0; 
        loop {
            if count == 9 {break;}
            count +=1;
            println!("from infinite loop with break ");
        }


        for (i,j) in (5..10).enumerate() {
            println!("i = {} and j = {}", i, j);
        }  
    //infinite loop 
 //   loop {
 //       println!("from infinite loop");
  //  }
}
