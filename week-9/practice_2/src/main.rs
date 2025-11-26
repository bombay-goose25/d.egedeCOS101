use std::fs;
fn main() {
    fs::remove_file("data.txt").expect("could not");
    println!("file is removed")
}
