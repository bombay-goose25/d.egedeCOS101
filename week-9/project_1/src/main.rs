use std::fs::File;
use std::io::Write;

fn main(){
    let mut file = File::create("categories.txt").expect("create failed");

    file.write_all("\nNigerian Breweries Product Categories".as_bytes()).expect("write failed");
    file.write_all("\n-----------------------------------".as_bytes()).expect("write failed");

    file.write_all("\nLAGER:\n".as_bytes());
    let lager = " 33 Export\n Desperados\n Goldberg\n Gulder\n Heineken\n Star\n";
        file.write_all(lager.as_bytes()).expect("write failed");

    file.write_all("\nSTOUT:\n".as_bytes()).expect("write failed");
    let stout = " Legend\n Turbo King\n Williams\n";
    
    file.write_all(stout.as_bytes()).expect("write failed");

    file.write_all("\nNON-ALCOHOLIC:\n".as_bytes());
    let non_alcoholic = " Maltina\n Amstel Malta\n Malta Gold\n Fayrouz\n";
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("categories.txt created successfully.");

}
