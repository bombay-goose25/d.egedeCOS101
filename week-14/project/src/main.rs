use std::fs::File;
use std::io::Read;
use std::io;

fn main(){
    let mut input6 = String::new();
    println!("Level of management:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let user = input6.trim();
    authorize(user);
    }

fn authorize(role: &str) {
    if role == "administrator" {
        openandread("globacom_dbase.sql");
    } 
    else if role == "project_manager" {
        openandread("project_tb.sql");
    } 
    else if role == "employee" {
        openandread("staff_tb.sql");
    } 
    else if role == "customer" {
        openandread("customer_tb.sql");
    } 
    else if role == "vendor" {
        openandread("dataplan_tb.sql");
    } 
    else {
        println!("Access Denied");
    }

}


fn openandread(file: &str) {
    let file = File::open(file);
    let mut contents = String::new();
    file.expect("failed to open").read_to_string(&mut contents);
    println!("ACCESS GRANTED\n{}", contents);

}
