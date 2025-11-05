use std::io;
fn main() {
    loop {
        let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Welcome to Teccna Bookshop
        \n Code      Book title                 (N)Price
        \n----------------------------------------------
        \n R        'Rust for beginners'         15000
        \n----------------------------------------------
        \n A        'AI Basics'                  12500
        \n----------------------------------------------
        \n D        'Data structures in rust'    20000
        \n----------------------------------------------
        \n N        'Networking essentials'      18000
        \n----------------------------------------------");

    println!("Enter book code:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let code = input1.trim();

    println!("Enter quantity:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quan:i32 = input2.trim().parse().expect("Enter a valid input");

    let amnt1:i32 = quan * 15000;
    let amnt2:i32 = quan * 12500;
    let amnt3:i32 = quan * 20000;
    let amnt4:i32 = quan * 18000;


    if code == "R"{
        println!("Rust for beginners x {}",quan );
        println!("Total cost: N{}",amnt1 );
    }else if code == "A"{
        println!("AI Basics x {}",quan );
        println!("Total cost: N{}",amnt2 );
    }else if code == "D"{
        println!("Data structures in rust x {}",quan );
        println!("Total cost: N{}",amnt3 );
    }else if code == "N"{
        println!("Networking essentials x {}",quan );
        println!("Total cost: N{}",amnt4 );
    }
    let dis:i32 = (amnt1 * 7) / 100;
        let dis2:i32 = amnt1 - dis;

    let dis3:i32 = (amnt2 * 7) / 100;
        let dis4:i32 = amnt2 - dis3;

        let dis5:i32 = (amnt3 * 7) / 100;
        let dis6:i32 = amnt3 - dis5;

        let dis7:i32 = (amnt4 * 7) / 100;
        let dis8:i32 = amnt4 - dis7;
    if quan > 3 {
        
        println!("Amount payable with discount: N{}",dis2 );
    }else if quan > 3 {
        
        println!("Amount payable with discount: N{}",dis4 );
    }else if quan > 3 {
        
        println!("Amount payable with discount: N{}",dis6 );
    }else if quan > 3 {
        
        println!("Amount payable with discount: N{}",dis8 );
    }
    println!("Enter (yes/no) for a new customer:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let answer = input3.trim();

    if answer == "no"{
        break;
    }
    }

}
