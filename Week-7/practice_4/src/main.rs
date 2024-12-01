use std::io;

fn add(a: i64, b: i64) {
    let sum = a + b;

    println!("Sum of A and B = {}", sum);
}

fn main() {

    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parametr B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i64 = input2.trim().parse().expect("Invalid input");

    // call add function with arguments
    add(a, b);
}