use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

struct Student {
    name:String,
    mat_no:String,
    dep:String,
    lvl:String,
}
fn main() {
    loop {
        let mut a:Vec<Student> = Vec::new();

    let mut name = String::new();

    println!("Enter Student name:");
    io::stdin().read_line(&mut name).unwrap();



    let mut mat_no = String::new();

    println!("Enter Mat no:");
    io::stdin().read_line(&mut mat_no).unwrap();

    let mut dep = String::new();

    println!("Enter Department:");
    io::stdin().read_line(&mut dep).unwrap();
    
    

    let mut lvl = String::new();

    println!("Enter Level:");
    io::stdin().read_line(&mut lvl).unwrap();
   

    a.push(Student{name:name.trim().to_string(),
                   mat_no:mat_no.trim().to_string(),
                   dep:dep.trim().to_string(),
                   lvl:lvl.trim().to_string(),});

    let mut file = OpenOptions::new().append(true).open("student.csv").expect("Failed to open");
    for i in &a{
        let record = format!("{} {} {} {}\n",i.name,i.mat_no,i.dep,i.lvl);
        
    }
    
    println!("Data recorded successfully");
    let mut input = String::new();
    println!("Would you like to enter another student's details?[Y/N]");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let answer:char = input.trim().parse().expect("Enter valid input");
    if answer == 'N'{
        break;
    }else if answer == 'Y'{
        continue;
    }else{
        println!("Enter a valid choice[Y/N]");
        break;
       
    }
    }

}

