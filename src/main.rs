use std::io;
fn fahrenheitchange(number:i32){
    let x:i32 = number * 9/5 + 32;
    println!("{} Degree Fahrenheit",x);
    if x >= 104 {
        println!("Did Your Omelette Fried Enough In Your Car Sir ?");
    } else if x <=32 {
        println!("Make Sure To Set a Place For Your Snowman Sir")
    } else {
        println!("Sunny Day Sir !");
    }
    println!();
    println!();
}
fn celciuschange(number:f64) {
    let x:f64 = number - 32.0;
    let y:f64 = 1.8;
    let z:f64 = x/y;
    println!("{} Degree Celsius",z);
    if z>= 40.0 {
        println!("Did Your Omelette Fried Enough In Your Car Sir ?");
    } else if z <=0.0{
        println!("Make Sure To Set a Place For Your Snowman Sir")
    } else {
        println!("Sunny Day Sir !");
    }
    println!();
    println!();   
}
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Celcius & Fahrenheit Calc");
    println!();
    println!("Enter 1 For Fahrenheit Result");
    println!("Enter 2 For Celcius Result");
    println!();
    io::stdin().read_line(&mut input1).unwrap();
    println!("Enter Your Value Based on Your Choise");
    io::stdin().read_line(&mut input2).unwrap();
    let choose:i32 = input1.trim().parse().unwrap();

    let result1:i32 = input2.trim().parse().unwrap();
    if choose == 1 {
        fahrenheitchange(result1);
    } else if choose == 2 {
        celciuschange(result1.into());
    }
}