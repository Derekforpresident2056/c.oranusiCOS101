use std::io;


fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    

    let v = vec!["none", "office administrator", "academic", "lawyer", "teacher"];

    let vo = vec!["none","intern", "administrator", "senior administrator", "office manager", "director","CEO"];
    let va = vec!["none","research assistant","PhD candidate","post-doc researcher","senior lecturer","Dean"];
    let vl = vec!["none","paralegal;","junior associate","associate","senior associate1-2","senior associate3-4","partner"];
    let vt = vec!["none","placement","classroom teacher","snr teacher","leading teacher","deputy principal","principal"];
    let vp = vec!["none","APS1-2","APS3-5","APS5-8","EL18-10","EL210-13","SES"];

    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("not a valid string");

    println!("enter the digit that corresponds to your field");
    println!("none --0");
    println!("office administrator --1");
    println!("academic --2");
    println!("lawyer --3");
    println!("teacher --4");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let field:usize = input2.trim().parse().expect("nota valid number");

    if field == 0
    {
        println!("unavailable");
    }
    else if field == 1
    {
        println!("enter your position, use the corresponding number");
        println!("none --0");
        println!("intern --1");
        println!("administrator --2");
        println!("senior administrator --3");
        println!("office manager --4");
        println!("director --5");
        println!("CEO --6");
        
        
    }
    else if field == 2
    {
        println!("enter your position, use the corresponding number");
        println!("none --0");
        println!("research assistant --1");
        println!("Phd candidate --2");
        println!("post doc researcher --3");
        println!("senior lecture --4");
        println!("dean --5");
        
        
    }
    else if field == 3
    {
        println!("enter your position, use the corresponding number");
        println!("none --0");
        println!("paralegal --1");
        println!("junior associate --2");
        println!("associate --3");
        println!("senior associate[1-2] --4");
        println!("senior associate[3-4] --5");
        println!("partner --6");
       
        
    }
    else if field == 4
    {
        println!("enter your position, use the corresponding number");
        println!("none --0");
        println!("placement --1");
        println!("classroom teacher --2");
        println!("snr teacher --3");
        println!("leading teacher --4");
        println!("deputy principal --5");
        println!("principal --6");
        
    }
    else
    {
        println!("invalid");
    }


    io::stdin().read_line(&mut input3).expect("not a valid string");
    let gta:usize = input3.trim().parse().expect("nota valid number");



    let efield: &str = v[field];

    if field == 1
    {
        let eposition: &str = vo[gta];
        println!("{}",input1 );
        println!("you work as a/an {} in the field '{}' ",eposition,efield );
        println!("Your current APS level is{}",vp[gta] );
    }
    else if field == 2
    {
        let eposition: &str = va[gta];
        println!("{}",input1 );
        println!("you work as a/an {} in the field '{}' ",eposition,efield );
        println!("Your current APS level is{}",vp[gta] );
    }
    else if field == 3
    {
        let eposition: &str = vl[gta];
        println!("{}",input1 );
        println!("you work as a/an {} in the field '{}' ",eposition,efield );
        println!("Your current APS level is{}",vp[gta] );
    }
    else if field == 4
    {
        let eposition: &str = vt[gta];
        println!("{}",input1 );
        println!("you work as a/an {} in the field '{}' ",eposition,efield );
        println!("Your current APS level is{}",vp[gta] );
    }
    else
    {
        println!("invalid");
    }
}

    