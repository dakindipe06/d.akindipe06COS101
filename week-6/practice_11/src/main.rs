fn main() {
    let a:i64 = 2;     // Bit presentation 10
    let b:i64 = 3;     // Bit presentation 11

    let mut result:i64;

    result = a & b;
    println!("(a & b) => {} ",result);

    result = a | b;
    println!("(a | b) => {} ",result);

    result = a ^ b;
    println!("(a ^ b) => {} ",result);

    result = !b;
    println!("(!b) => {} ",result);

    result = a << b;
    println!("(a << b) => {} ",result);

    result = a >> b;
    println!("(a >> b) => {} ",result);
}
