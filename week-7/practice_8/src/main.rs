fn main() {
    let city_arr1:[&str;5] = ["Abuja","PH","Maiduguri","Kano","Lagos"];
    println!("array is {:?}",city_arr1 );
    println!("array size is :{}",city_arr1.len());

    for index in 0..5{
        println!("City index {} is located in : {}",index,city_arr1[index] );
    }
}
