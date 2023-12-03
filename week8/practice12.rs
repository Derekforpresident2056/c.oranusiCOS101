fn main() {

let mut colors = ["red","green","yellow","white"];
println!("\noriginal array = {:?}",colors );
let slicedcolors = &mut colors[1..3];
println!("first slice = {:?}",slicedcolors );
slicedcolors[1] = "purple";
println!("changed slice = {:?}",sliced colors );

}