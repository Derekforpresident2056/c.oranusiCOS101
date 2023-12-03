fn main() {
   let name = vec!["mary","sally","sam","greg","ade","mark","june","ife"];
   let age = vec![16,17,18,15,21,12,34,23];
   println!("\nAGE ALLOCATION\n");

   for i in 0..age.len(){
    println!("{} is {} years old\n",name[i],age[i]);
   }
}