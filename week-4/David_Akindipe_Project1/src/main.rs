// rust program to  calculate the roots of a quardratic equation

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");
    let a:f64 = input1.trim().parse().expect("Not a vaild number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");


// Calculate the discriminant
let discriminant = b * b - 4.0 * a * c;

// Determine the nature of the roots based on the discriminant
if discriminant > 0.0 {
    let root1:f64 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2:f64 = (-b + discriminant.sqrt()) / (2.0 * a);
    println!("The roots are real and distinct: root1 = {:.2}, root2 = {:.2}", root1, root2);
} else if discriminant == 0.0 {
    let root:f64 = -b / (2.0 * a);
    println!("The equation has one real root: root = {:.2}", root);
} else  {
    println!("The equation has no real roots.");
}
}
