fn main() {
    
    let b:(i64,bool,f64) = (110,true,10.9);
    print(b);

}

//pass the tuple as a parameter
fn print(x:(i64,bool,f64)) {

    println!("Inside print method");
    println!("{:?}",x);

}
