use std::io;

fn main() {

	

	
    let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();

    let x = 0;
	loop{
		println!("Hi, please enter your name");
	io::stdin().read_line(&mut input1).expect("not a valid string");

	println!("whats your email address");
	io::stdin().read_line(&mut input2).expect("not a valid string");

	println!("whats your department");
	io::stdin().read_line(&mut input3).expect("not a valid string");

	println!("what level woukld that be, 100 - 400");
	io::stdin().read_line(&mut input4).expect("not a valid string");
	let level:i32 = input4.trim().parse().expect("nota valid number");

	println!("whats your state of origin");
	io::stdin().read_line(&mut input5).expect("not a valid string");

	println!("are you a class rep");
	println!("0--yes");
	println!("1--no");
	io::stdin().read_line(&mut input6).expect("not a valid string");
	let rep:i32 = input6.trim().parse().expect("nota valid number");

	println!("whats your CGPA");
	io::stdin().read_line(&mut input7).expect("not a valid string");
	let gpa:f32 = input7.trim().parse().expect("nota valid number");

	if rep == 0 && level > 100 && gpa > 4.0
	{
		println!("name:{}  email:{}  department:{}  state of origin:{}",input1,input2,input3,input5 );
		println!("YOU CAN VOTE");
	}
	else
	{
		println!("sorry, you are not eligible to vote");
	} 

	if x == 150{
		break;
	}

	}

	
}





