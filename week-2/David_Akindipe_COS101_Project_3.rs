fn main() {
	let p:f64 = 210000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//compound interest
	let a = p * (1.0 - (r / 100.0)).powf(n);
	println!("Amount is {}", a);
	let ci = p - a;
	println!("Compound Interest is {}", ci);

}

