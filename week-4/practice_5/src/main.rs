use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height in cm:");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0{
        println!("You are of avg");
        }else if height > 170.0 && height <= 195.0{
        println!("You are tall");
    }else if height < 150.0 && height > 100.0{
        println!("Youre short");
    } else {
        println!("Abnormal height");
    }
    
}
