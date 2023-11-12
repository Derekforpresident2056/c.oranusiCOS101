use std::io;

fn main() {
	
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Enter your name");
	io::stdin().read_line(&mut input1).expect("not a valid string");

	println!("Enter your age {}",input1);
	io::stdin().read_line(&mut input2).expect("not a valid string");
	let age:i32 = input2.trim().parse().expect("nota valid number");

	if age  >= 40
    	{
    		println!("your incentive is 1560000");
    	}
    	else if age >= 30 && age < 40
        {
    		println!("your incentive is 1460000");
    	}
    	else if age >= 25 && age < 30
    	{
    		println!("your incentive is 1300000");
    	}

    println!("do you have any experience prior to this job,enter 1 for yes and 2 for no");
    io::stdin().read_line(&mut input3).expect("not a valid string");
	let detr:i32 = input3.trim().parse().expect("nota valid number");

	if detr == 1
	{
		println!("lovely, you can work with the incentive stated above");
	}
	else if detr == 2
	{
		println!("im sorry but you can only earn 100000, we hope thats fine with you");
	}
	else if detr > 2
	{
		println!("invalid response");
	}

	
    	
    

}
    
