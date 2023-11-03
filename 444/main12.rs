use std::io;

fn main() {
    println!("\nstudent information management system");

    println!("\npleae enter your name");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("failed to read input");
    println!("your name is: {}", name);

    println!("\nenter your age");
    let mut age  = String::new();
    io::stdin().read_line(&mut age).expect("failed to read input");
    let age:i32 = age.trim().parse().expect("ïnput not an integer");
    println!("your age is: {}", age);


}
