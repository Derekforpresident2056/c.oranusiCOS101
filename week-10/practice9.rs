struct Rectangle{
    width:u32, height:u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
}

fn main(){
    let small = Rectangle {
        width:10,
        height:20
    };

    println!("width is {} and height is {} \n area must be {}",small.width,small.height,small.area());
}
|| input2.len() < 3 && input2.len() > 8
|| ftg.is_uppercase()