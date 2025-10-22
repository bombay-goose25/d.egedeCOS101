use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a");
    io::stdin().read_line(&mut input1).expect("Unable to read input");
    let a:f64 = input1.trim().parse().expect("Enter a valid no");

    println!("Enter b");
    io::stdin().read_line(&mut input2).expect("Unable to read input");
    let b:f64 = input2.trim().parse().expect("Enter a valid no");

    println!("Enter c");
    io::stdin().read_line(&mut input3).expect("Unable to read input");
    let c:f64 = input3.trim().parse().expect("Enter a valid no");

    let discriminant:f64 = b * b - 4.0 * a * c ;
    if discriminant > 0.0{
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The distinct and real roots are {} and {}",root1,root2 );
    } else if discriminant ==0.0{
        let root = (-b) / (2.0 * a);
        println!("One real root {}", root);
    }else {
        println!("No real roots");
    }

}
