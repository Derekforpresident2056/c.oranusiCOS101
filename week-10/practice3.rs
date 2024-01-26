fn main() {
   let v = vec![20,30,40,50];
   let v2 = vec;
   let v2_return = display(v2);
   println!("in main {:?}",v );

   fn display(v:Vec<i32>)->Vec<i32> {
    println!("inside display {:?}",v );
    return v;
   }

