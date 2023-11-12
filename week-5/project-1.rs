use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("enter A: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");

    println!("enter B: ");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");

    println!("enter C: ");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let c:f32 = input3.trim().parse().expect("not a valid number");

  
    let b2 = f32::powf(b,2.0);

    let discriminant:f32 = b2 - (4.0 * a * c);

    if discriminant > 0.0 
    {
    	let root1:f32 = ((-b) + discriminant.sqrt()) / 2.0 * a;
    	let root2:f32 = ((-b) - discriminant.sqrt()) / 2.0 * a;
    	println!("the roots are {} and {}",root1,root2); 
    }
    else if discriminant == 0.0
    {
        let root1:f32 = (-b) / (2.0 * a);
        let root2:f32 = (-b) / (2.0 * a);
        println!("the roots are {} and {}",root1,root2); 
    }
    else if discriminant < 0.0
    {
        println!("there are no real roots");
    }
}    




