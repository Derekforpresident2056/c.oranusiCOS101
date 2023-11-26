use std::io;


fn area_trapezium(a: i32, b: i32, c: i32) {
    let area_trapezium = ((a / 2) * (b + c));
    println!(" the area is{}",area_trapezium);
}

fn area_rhombus(a:i32, b:i32) {
    let area_rhombus =   ((a * b) / 2);
    println!("the area is{}",area_rhombus);
}

fn area_parallelogram(a:i32, b:i32) {
    let area_parallelogram = a * b;
    println!("the area is{}",area_parallelogram);
}

fn area_cube(a:i32) {
    let area_cube = (6 * a.pow(2));
    println!("the area is{}",area_cube);
}

fn volume_cylinder(a:i32, b:i32) {
    let volume_cylinder = ((22 / 7) * a.pow(2) * (b));
    println!("the area is{}",volume_cylinder);
}

fn main() {
    println!("what formula is required");
    println!("enter the corresponding digit");
    println!("area of trapezium ---1");
    println!("area of rhombus ---2");
    println!("area of parallelogram ---3");
    println!("area of cube ---4");
    println!("area of cylinder ---5");

    let mut input = String::new();
    println!("so which will it be");
    io::stdin().read_line(&mut input).expect("failed to read");
    let formula:i32 = input.trim().parse().expect("invalid input");

    if formula == 1
    {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new(); 

        println!("enter height");
        io::stdin().read_line(&mut input1).expect("failed to read");
        let a:i32 = input1.trim().parse().expect("invalid input");

        println!("enter a");
        io::stdin().read_line(&mut input2).expect("failed to read");
        let b:i32 = input2.trim().parse().expect("invalid input");

        println!("enter b");
        io::stdin().read_line(&mut input3).expect("failed to read");
        let c:i32 = input3.trim().parse().expect("invalid input");

        area_trapezium(a , b , c);
    }
    else if formula == 2
    {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("enter a");
        io::stdin().read_line(&mut input1).expect("failed to read");
        let a:i32 = input1.trim().parse().expect("invalid input");

        println!("enter b");
        io::stdin().read_line(&mut input2).expect("failed to read");
        let b:i32 = input2.trim().parse().expect("invalid input");

        area_rhombus(a , b);
    }
    else if formula == 3
    {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("enter a");
        io::stdin().read_line(&mut input1).expect("failed to read");
        let a:i32 = input1.trim().parse().expect("invalid input");

        println!("enter b");
        io::stdin().read_line(&mut input2).expect("failed to read");
        let b:i32 = input2.trim().parse().expect("invalid input");
        
        area_parallelogram(a , b);
    }
    else if formula == 4
    {
        let mut input1 = String::new();
        
        println!("enter a");
        io::stdin().read_line(&mut input1).expect("failed to read");
        let a:i32 = input1.trim().parse().expect("invalid input");

        area_cube(a);
    }
    else if formula == 5
    {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("enter a");
        io::stdin().read_line(&mut input1).expect("failed to read");
        let a:i32 = input1.trim().parse().expect("invalid input");

        println!("enter b");
        io::stdin().read_line(&mut input2).expect("failed to read");
        let b:i32 = input2.trim().parse().expect("invalid input");

        volume_cylinder(a , b);

    }
}