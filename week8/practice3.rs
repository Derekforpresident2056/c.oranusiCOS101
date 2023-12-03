fn value(n:Option<&char>)
{
    println!("element of vector {:?}",n);
}

fn main(){
    let v = vec!['R','U','S','T','A','C','E','A','N'];

    let mut input1 = String::new();
    println!("\nEnter an index value between 0 and 8");
    std::io::stdin().read_line(&mut input1).expect("failed to read input");
    let index:usize = input1.trim().parse().expect("not a valid input");

    let ch: Option<&char> = v.get(index);
    value(ch);
}
    
