use std::io;
use std::io::Read;

fn admin() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn project() {
    let mut file = std::fs::File::open("project.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn employee() {
    let mut file = std::fs::File::open("employee.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn dataplan() {
    let mut file = std::fs::File::open("dataplan.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn main() {
    println!("What position do you hold, select from the list below");
    println!("                                                     ");
    println!("ADMINISTRATOR----1");
    println!("PROJECT MANAGER----2");
    println!("EMPLOYEE----3");
    println!("CUSTOMER----4");
    println!("VENDOR----5");
    println!("           ");

    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let pos:i32 = input.trim().parse().expect("invalid input");

    if pos == 1
    {
        admin();
    }
    else if pos == 2
    {
        project();
    }
    else if pos == 3
    {
        employee();
    }
    else if pos == 4
    {
        customer();
    }
    else if pos == 5
    {
        dataplan();
    }
    else
    {
        println!("invalid response");
    }

}