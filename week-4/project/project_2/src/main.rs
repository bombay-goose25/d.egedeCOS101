use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("is the person experienced?(Y/N)", );
    io::stdin().read_line(&mut input1).expect("Unable to read input");
    let exp = input1.trim();

    println!("Enter your age", );
    io::stdin().read_line(&mut input2).expect("Unable to read input");
    let age:i32 = input2.trim().parse().expect("Enter a valid answer");

    let incentive:i32;

    if exp == "Y" {
        if age >= 40 {
            incentive = 1_560_000;
        }else if age >= 30 {
            incentive = 1_480_000;
        }else if age < 28 {
            incentive = 1_300_000;
        }else{
            incentive = 1_100_000;
        }
    }else{
        incentive = 100_000;
    }
    println!("The incentive is {}",incentive );
}

