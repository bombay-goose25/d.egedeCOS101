use std::io;

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().unwrap()
}


fn trapezium() {
    let h = read_input("Enter height:");
    let b1 = read_input("Enter base 1:");
    let b2 = read_input("Enter base 2:");
    let area = (h / 2.0) * (b1 + b2);
    println!("Area of Trapezium = {}", area);
}

fn rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

fn parallelogram() {
    let base = read_input("Enter base:");
    let height = read_input("Enter height:");
    let area = base * height;
    println!("Area of Parallelogram = {}", area);
}

fn cube() {
    let side = read_input("Enter side length:");
    let area = 6.0 * side * side;
    println!("Area of Cube = {}", area);
}

fn cylinder() {
    let r = read_input("Enter radius:");
    let h = read_input("Enter height:");
    let volume = std::f64::consts::PI * r * r * h;
    println!("Volume of Cylinder = {}", volume);
}

fn main() {
    println!("Choose a calculation:");
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice: usize = choice.trim().parse().unwrap();

    let operations: [fn(); 5] = [trapezium, rhombus, parallelogram, cube, cylinder];

    if choice >= 1 && choice <= 5 {
        operations[choice - 1](); 
    } else {
        println!("Enter a valid choice!");
    }
}

