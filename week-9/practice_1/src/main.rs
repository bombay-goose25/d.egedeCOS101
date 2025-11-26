use std::io::Write;
fn main() {
    let announce = "Wk-9 rust file input & output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("welcome to rust programming\n"
.as_bytes()).expect("write faild");
file.write_all(announce.as_bytes()).expect("write failed");
file.write_all(dept.as_bytes()).expect("write failed");
println!("\ndata written to file");
}
