fn main() {
   let x = vec![100,200,300];
   borrow_vector(&x);

   println!("printing yhe value from main() x[0]={}",x[0]);
   println!("*******************************************");

   borrow_vector(z:&Vec<i32>){
    println!("******************************************");
    println!("inside print vector function {:?}",z );
    println!("-----------------------------------------");
   }
