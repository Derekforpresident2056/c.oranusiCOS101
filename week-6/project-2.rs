use std::io;

fn main() {

    let mut input1 = String::new();
	let mut input2 = String::new();

	let x = 0;
	loop{
		
		println!("enter your name please");
		io::stdin().read_line(&mut input1).expect("not a valid string");

		println!("how many papers have you published");
		io::stdin().read_line(&mut input2).expect("not a valid string");
		let papers:i32 = input2.trim().parse().expect("nota valid number");

		if papers > 2 && papers < 6
		{
		println!("NAME:{} INCENTIVE:N500000,",input1 );
        }
		else if papers > 5 && papers < 10
		{
		println!("NAME:{} INCENTIVE:N800000",input1 );
        }
		else if papers > 9
		{
		println!("NAME:{} INCENTIVE:N1000000",input1 );
        }
		else if papers < 3
		{
		println!("NAME:{} INCENTIVE:N100000",input1 );
	    }

	    if x > 500{
	    	break
	    }
    }
}
