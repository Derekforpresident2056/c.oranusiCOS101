struct Employee{
    ceo:String,
    company:String,
    age:u32
}
fn main(){
    let emp1 = Employee {
        company:String::from("microsoft"),
        ceo:String::from("satya nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("google"),
        ceo:String::from("sundai pincha"),
        age:56
    };
    display(emp1);
    display(emp2);
}

fn display(emp:Employee){
    println!("name is {}, company is {}, age is {}",emp.ceo,emp.company,emp.age );
}