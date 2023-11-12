use std::io;

  fn main() {
    
    let mut input1 = String ::new();
    let mut input2 = String ::new();

    println!("enter the speed of the car: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");

     println!("enter the time taken: ");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");

    let kilometers = a / 0.621;
    println!("distance in kilometers is {}",kilometers);

    let speed = kilometers / b;
    println!("speed of the car is {}",speed);


}
