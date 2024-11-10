// Rust program to calculate the annual incentive based on the experience and age of an employee

use std::io;

fn main() {

    // Get input for experience
    println!("Is the employee experienced? (yes or no): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experienced = experience.trim().to_lowercase() == "yes";

    // Get input for age
    println!("Enter the age of the employee: ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u64 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid age entered.");
            return;
        }
    };
        // Calculate incentive based on conditions
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0 // Incentive amount not specified for age between 28 and 30
        }
    } else {
        100_000
    };

    // Display the incentive
    if incentive != 0 {
        println!("The annual incentive for the employee is: N{}", incentive);
    } else {
        println!("No specific incentive set for employees of age {} and experience level provided.", age);
    }
} 