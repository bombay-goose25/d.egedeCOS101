fn main() {
	let p = 510000.0;
	let r = 5.0;
	let n = 3.0;
	let a = p*(1.0 - r/100.0).powf(n);
	println!("Value after depreciation = {}",a);
}