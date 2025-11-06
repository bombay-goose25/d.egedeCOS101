use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Pick the type of food desired by the letters
        \n        Menu                            Price
        \nP=Poundo Yam/Edinkaiko Soup           -N3,200
        \nF=Fried Rice and Chicken              -N3,000
        \nA=Amala & Ewedu Soup                  -N2,500
        \nE=Eba & Egusi Soup                    -N2,000
        \nW=White Rice and Stew                 -N2,500");

    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let code:&str = input1.trim();

    println!("Enter quantity:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quan:i32 = input2.trim().parse().expect("Enter valid input");

    let mut price:i32 = 0;
    

    if code == "P"{
        price = 3200;
        
        
    }else if code == "F"{
        price = 3000;
        
    }else if code == "A"{
        price = 2500;
        
    }else if code == "E"{
        price = 2000;
        
    }else if code == "W"{
        price = 2500;
        
    }
    let total = price * quan;
    println!("Code: {}",code );
    println!("Quantity: x {}",quan );
    println!("Total price: N{}", total);

    if total > 10000{
        let dis = (total * 5) / 100;
        let dis2 = total - dis;
        println!("Amount with 5% discount applied: N{}",dis2 );
    }else {
        println!("No discount applied");
    }

}
