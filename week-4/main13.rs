use std::io;

fn main() {
    
    let mut input1 = String ::new();
    let mut input2 = String ::new();
    let mut input3 = String ::new();

    println!("enter first edge of triangle: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");

    println!("enter first edge of triangle: ");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");

    println!("enter first edge of triangle: ");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let c:f32 = input3.trim().parse().expect("not a valid number");

    let s:f32 = (a + b + c) / 2.0;
    let mut area = s * ( s - a) * (s - b) * (s -c);
    area = area.sqrt();
    println!("area of a triangle is: {}", area);

}

