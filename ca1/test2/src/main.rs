use std::io;
fn main() {
    loop {
        let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter employee name:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _name:&str = input1.trim();

    println!("Enter hours worked:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let hours:i32 = input2.trim().parse().expect("Enter a valid input");

    let extra:i32 = hours - 40;
    let rate2:i32 = extra * 4500;
    let rate:i32 = hours * 3000;
    let rate3:i32 = 40 * 3000;
    let salary:i32 = rate3 + rate2;

    if hours <= 40{
        
        println!("Hourly pay: N{}",rate );
        println!("Total salary: N{}",rate );
    }else if hours >= 40{
        println!("Hourly pay: N{}", rate3);
        println!("Extra hours: {}", extra);
        println!("Extra hourly pay: N{}",rate2 );
        
        println!("Total salary: N{}", salary);
    }
    
    let vat:i32 = salary - 2000;
    if salary > 100000{
        println!("Total salary with tax: N{}",vat );
    }
    println!("Would you like to calculate another(yes/no)");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let answer = input3.trim();

    if answer == "no"{
        break
    }
    }
}
