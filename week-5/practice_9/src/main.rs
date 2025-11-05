fn main() {
    let a:i32 = 10;
    let b:i32 = 2;

    println!("Value of A :{}",a );
    println!("Value of B :{}",b );

    let res = a>b;
    println!("A greater than B:{}",res );

    let res = a<b;
    println!("A less than B:{}",res );

    let res = a>=b;
    println!("A greater than or equal to B:{}",res );

    let res = a<=b;
    println!("A less than or equal to B:{}",res );

    let res = a==b;
    println!("A equal to B:{}",res );

    let res = a!=b;
    println!("A not equal to:{}",res );
}
