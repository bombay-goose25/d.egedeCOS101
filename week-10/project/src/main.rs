#[derive(Clone)]
struct Brand {
    hp:f64,
    ibm:f64,
    toshiba:f64,
    dell:f64
}
impl Brand {
    fn total(&self)->f64{
        (self.hp * 3.0) + (self.ibm * 3.0) + (self.toshiba * 3.0) + (self.dell * 3.0)
    }
}
fn main() {
    let brands = Brand {
        hp:650000.00,
        ibm:755000.00,
        toshiba:550000.00,
        dell:850000.00
    };
    println!("
        HP Laptop - N{}\n
        IBM Laptop - N{}\n
        Toshiba Laptop - N{}\n
        Dell Laptop - N{}\n-------------------------------------",brands.hp,brands.ibm,brands.toshiba,brands.dell);
    brandz(brands.clone());
    println!("
-------------------------------------
Total amount payable: N{}", brands.total());
}

fn brandz(laptop_1:Brand){
    let a:f64 = laptop_1.hp * 3.0;
    println!("HP Laptop x 3 : N{}",a );
    let b:f64 = laptop_1.ibm * 3.0;
    println!("IBM Laptop x 3 : N{}",b );
    let c:f64 = laptop_1.toshiba * 3.0;
    println!("Toshiba Laptop x 3 : N{}",c );
    let d:f64 = laptop_1.dell * 3.0;
    println!("Dell Laptop x 3 : N{}",d );

}

