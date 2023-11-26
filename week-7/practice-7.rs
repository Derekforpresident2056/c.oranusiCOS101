fn main(){

    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("\narray is {:?}", arr1);
    println!("\narray size is; {}",arr1.len());
    

    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\nArray without data type");
    println!("\narray is {:?}", arr2);
    println!("\narray size is; {}",arr2.len());

    let arr3:[i32;8] = [-1;8];
    println!("\nArray with different data values");
    println!("\narray is {:?}", arr3);
    println!("\narray size is; {}",arr3.len());




}