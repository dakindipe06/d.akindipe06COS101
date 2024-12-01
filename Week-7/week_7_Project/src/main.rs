use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) {
    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {:.2}", area);
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) {
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {:.2}", area);
}

fn area_of_parallelogram(base: f64, height: f64) {
    let area = base * height;
    println!("Area of Parallelogram = {:.2}", area);
}

fn area_of_cube(side: f64) {
    let area = 6.0 * side.powi(2);
    println!("Area of Cube = {:.2}", area);
}

fn volume_of_cylinder(radius: f64, height: f64) {
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("Volume of Cylinder = {:.2}", volume);
}

fn main() {
    println!("Choose a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    println!("Enter your choice:");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u64 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => {
            let height = read_f64("Enter height:");
            let base1 = read_f64("Enter base1:");
            let base2 = read_f64("Enter base2:");
            area_of_trapezium(height, base1, base2);
        }
        2 => {
            let diagonal1 = read_f64("Enter diagonal1:");
            let diagonal2 = read_f64("Enter diagonal2:");
            area_of_rhombus(diagonal1, diagonal2);
        }
        3 => {
            let base = read_f64("Enter base:");
            let height = read_f64("Enter height:");
            area_of_parallelogram(base, height);
        }
        4 => {
            let side = read_f64("Enter side length:");
            area_of_cube(side);
        }
        5 => {
            let radius = read_f64("Enter radius:");
            let height = read_f64("Enter height:");
            volume_of_cylinder(radius, height);
        }
        _ => println!("Invalid choice!"),
    }
}

fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}