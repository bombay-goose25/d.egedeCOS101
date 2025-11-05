use std::io;
fn main() {
    let cels:f32 = 0.0;
    let mut input1 = String::new();
    println!("Input the temperature in Celsius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let cels:f32 = input1.trim().parse().expect("Enter valid input");

    let faren:f32 = (9.0 / 5.0) * cels + 32.0;
    let kelv:f32 = cels + 273.15;
    println!("Your value entered: {}",cels );
    println!("Corresponding value of your input \nIn Farenheit: {} \nIn Kelvin: {}",faren,kelv );

    if cels < 0.0{
        println!("{} is below freezing point",cels);
    } else if cels <= 30.0 && cels >= 0.0{
        println!("{} is within normal range", cels);
    }else if cels > 30.0{
        println!("{} is a hot temperature", cels);
    }
}
