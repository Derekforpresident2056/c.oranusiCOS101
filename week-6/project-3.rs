use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
   

   

    println!("what times table would you like?");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let times_table:i32 = input1.trim().parse().expect("not a valid number");

    println!("what number would you like to stop at?");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let n:i32 = input2.trim().parse().expect("not a valid number");

    println!("The {} times table: ",times_table);
 let mut x = 0;
   loop{
    x+=1;
    let product = times_table * x;
    
    println!("{} x {} = {}",x,times_table,product);

    if x == n
    {
        break;
    }
   }
}
