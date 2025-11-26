use std::io;

fn main() {
    let mut c: Vec<(String, u32)> = Vec::new();

    let mut input = String::new();
    println!("How many candidates?");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut name = String::new();
        let mut years = String::new();

        println!("Enter candidate name:");
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter years of experience:");
        io::stdin().read_line(&mut years).unwrap();

        let y: u32 = years.trim().parse().unwrap();
        c.push((name.trim().to_string(), y));
    }

    let mut max_yrs = 0;

    let mut max_name = String::new();

    let mut i = 0;
    while i < c.len() {
        let (ref name, years) = c[i];

        if years > max_yrs {
            max_yrs = years;
            max_name = name.to_string();
        }

        i += 1;
    }

    println!("\nMost experienced: {} ({} years)", max_name, max_yrs);
}
