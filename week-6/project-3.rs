use std::io;

fn main() {
    let mut input1 = String::new();
    
    
    let start = 0;

   

    println!("enter number");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let n:i32 = input1.trim().parse().expect("nota valid number");

    

    for x in start .. (n + 1){

    
       
        println!("1 X {} = {}",x,x);
    }
}
