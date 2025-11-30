use std::io;
fn add(a:i32,b:i32) {
    let sum = a + b;
    println!("Sum of A and B = {}",sum);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter a");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d:i32 = input1.trim().parse().expect("Invalid");

    let mut input2 = String::new();
    println!("Enter b");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let e:i32 = input2.trim().parse().expect("Invalid");

    add(d,e);
}