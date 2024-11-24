use std::io;

fn main() {
    // Display menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup      - N3,200");
    println!("F = Fried Rice & Chicken          - N3,000");
    println!("A = Amala & Ewedu Soup            - N2,500");
    println!("E = Eba & Egusi Soup              - N2,000");
    println!("W = White Rice & Stew             - N2,500");

    // Prices
    let prices = [("P", 3_200), ("F", 3_000), ("A", 2_500), ("E", 2_000), ("W", 2_500)];

    // Input food type
    let mut food_type = String::new();
    println!("\nEnter the type of food (P/F/A/E/W): ");
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    // Validate food type
    let price = prices.iter().find(|&&(code, _)| code == food_type);
    if price.is_none() {
        println!("Invalid food type selected!");
        return;
    }
    let price = price.unwrap().1;

    // Input quantity
    let mut quantity = String::new();
    println!("Enter the quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: usize = match quantity.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity entered!");
            return;
        }
    };

    // Calculate total cost
    let total = price * quantity;
    let discount = if total > 10_000 { total / 20 } else { 0 };
    let final_total = total - discount;

    // Display the total charges
    println!("\n--- Order Summary ---");
    println!("Food Type: {}", food_type);
    println!("Quantity: {}", quantity);
    println!("Total: N{}", total);
    if discount > 0 {
        println!("Discount (5%): -N{}", discount);
    }
    println!("Final Total: N{}", final_total);
}
