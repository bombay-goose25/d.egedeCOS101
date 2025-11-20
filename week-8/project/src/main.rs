use std::io;

fn aps(role: &str, years: u32) -> String {
    let lawyer: Vec<(&str, &str)> = vec![
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-4"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL1 1-10"),
        ("Senior Associate 3-4", "EL2 10-11"),
        ("Partner", "SES"),
    ];

    let mut i = 0;
    while i < lawyer.len() {
        let (title, aps) = lawyer[i];

        if title == role && years >= 1 {
            return aps.to_string();
        }

        i += 1;
    }

    "Role not found".to_string()
}

fn main() {
    let mut role = String::new();
    let mut years = String::new();

    println!("Enter staff role EXACTLY as written:");
    std::io::stdin().read_line(&mut role).unwrap();

    println!("Enter years of experience:");
    std::io::stdin().read_line(&mut years).unwrap();

    let y: u32 = years.trim().parse().unwrap();
    let cleaned_role = role.trim();

    let level = aps(cleaned_role, y);

    println!("\nAPS Level: {}",level);
}
