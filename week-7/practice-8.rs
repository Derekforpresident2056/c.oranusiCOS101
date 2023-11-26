fn main() {
    let city_arr:[&str;5] = ["ABUJA", "PORT HARCOURT", "MAIDUGURI", "KANO", "LAGOS"];
    println!("ärray is {:?}",city_arr);
    println!("array sizee is {}",city_arr.len());
    for index in 0..5 {
        println!("city index {} is located in : {}",index,city_arr[index]);
    }
}