fn main() {
    let name = "Aisha";
    let uni:&str = "Pan atlantic uni";
    let address:&str = "km 52 lekki epe expressway, ibeju-lekki, lagos";
    println!("Name:{}",name );
    println!("Uni:{},\nAddress:{}",uni,address );

    let department:&'static str = "Computer science";
    let school:&'static str = "School ofComputer science";
    println!("Depart:{},\nSchool:{}",department,school );

}
