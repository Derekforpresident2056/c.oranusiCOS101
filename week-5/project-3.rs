use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();
	let mut input9 = String::new();
	let mut input10 = String::new();
	let mut input11 = String::new();
	let mut inputa = String::new();
	let mut inputb = String::new();
	let mut inputc = String::new();
	let mut inputd = String::new();
	let mut inpute = String::new();

    

    println!("enter your name");
    io::stdin().read_line(&mut input1).expect("not a valid string");

    println!("hello {} ,todays menu is,",input1);
    println!("P= poundo yam and edinkaiko soup @ n3200");
    println!("F= fried rice and chicken @ 3000");
    println!("A= amala and ewedu soup @ 2500");
    println!("E= eba and egusi soup @ 2000");
    println!("W= White rice and stew @ 2500");
    println!("Z= nothing, I wish to proceed,if asked for quantity,press any number to continue");
    println!("the code for each item in the menu is right after the ,@'on the menu");



    println!("what would you like today, please enter the corresponding letter");
    io::stdin().read_line(&mut inputa).expect("not a valid string");
    println!("what would you like today, please enter the corresponding code");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b1:i32 = input2.trim().parse().expect("not a valid number");
    println!("and how many would that be");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let a1:i32 = input3.trim().parse().expect("not a valid number");
    let price1 = a1 * b1;
    println!("{} orders of {}",a1,inputa );
    println!("that would cost you {}",price1 );




    println!("anything else, please enter the corresponding letter");
    io::stdin().read_line(&mut inputb).expect("not a valid string");
    println!("anything else, please enter the corresponding code");
    io::stdin().read_line(&mut input4).expect("not a valid string");
    let b2:i32 = input4.trim().parse().expect("not a valid number");
    println!("and how many would that be");
    io::stdin().read_line(&mut input5).expect("not a valid string");
    let a2:i32 = input5.trim().parse().expect("not a valid number");
    let price2 = a2 * b2;
    println!("{} orders of {}",a2,inputb );
    println!("that would cost you {}",price2 );



    println!("anything else, please enter the corresponding letter");
    io::stdin().read_line(&mut inputc).expect("not a valid string");
    println!("anything else, please enter the corresponding code");
    io::stdin().read_line(&mut input6).expect("not a valid string");
    let b3:i32 = input6.trim().parse().expect("not a valid number");
    println!("and how many would that be");
    io::stdin().read_line(&mut input7).expect("not a valid string");
    let a3:i32 = input7.trim().parse().expect("not a valid number");
    let price3 = a3 * b3;
    println!("{} orders of {}",a3,inputc );
    println!("that would cost you {}",price3 );



    println!("anything else, please enter the corresponding letter");
    io::stdin().read_line(&mut inputd).expect("not a valid string");
    println!("anything else, please enter the corresponding code");
    io::stdin().read_line(&mut input8).expect("not a valid string");
    let b4:i32 = input8.trim().parse().expect("not a valid number");
    println!("and how many would that be");
    io::stdin().read_line(&mut input9).expect("not a valid string");
    let a4:i32 = input9.trim().parse().expect("not a valid number");
    let price4 = a4 * b4;
    println!("{} orders of {}",a4,inputd );
    println!("that would cost you {}",price4 );



    println!("anything else, please enter the corresponding letter");
    io::stdin().read_line(&mut inpute).expect("not a valid string");
    println!("anything else, please enter the corresponding code");
    io::stdin().read_line(&mut input10).expect("not a valid string");
    let b5:i32 = input10.trim().parse().expect("not a valid number");
    println!("and how many would that be");
    io::stdin().read_line(&mut input11).expect("not a valid string");
    let a5:i32 = input11.trim().parse().expect("not a valid number");
    let price5 = a5 * b5;
    println!("{} orders of {}",a5,inpute );
    println!("that would cost you {}",price5 );



    let total = price1 + price2 + price3 + price4 + price5;
    println!("your total is {}",total );
    if total > 10000
    {
    	let discount = total * (95 / 100);
    	println!("congratulations, you'eligible for our discount, your grand total is {}",discount);
    }




}







    
    