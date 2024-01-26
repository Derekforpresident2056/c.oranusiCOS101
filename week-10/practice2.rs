fn main() {
    let v = vec!(101,102,103,104);
    let v2 = v;
    display(v2);
    println!("{:?}",v2);
}

 fn display(v:Vec<i32>){ 
    println!("inside display {:?}",v );
}
