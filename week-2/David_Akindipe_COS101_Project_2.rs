fn main(){
	let t:f64 = 2.0 * 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 3.0 * 750000.00;
	let d:f64 = 3.0 * 2850000.00;
	let a:f64 = 250000.00;

	//sum and average of sales recorded
	let sm = t + m + h + d + a;
	println!("Sum of sales recorded is {}", sm);
	let ave = (t + m + h + d + a)/(2.0 + 1.0 + 3.0 + 3.0 + 1.0);
	println!("Average of sales recorded is {}", ave);

}

