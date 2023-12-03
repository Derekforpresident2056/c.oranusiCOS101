fn main() {
   let datatype_tuple: (&str, f32, u8) = ("rust", 3.14, 100);
   println!("tuple contents = {:?}",datatype_tuple);

   let no_datatype_tuple  = ("rust", "fun", 100);
   println!("tuple contents = {:?}",no_datatype_tuple);

   println!("value at index o = {}",datatype_tuple.0);
   println!("value at index o = {}",datatype_tuple.1);
   println!("value at index o = {}",datatype_tuple.2);
}



   
