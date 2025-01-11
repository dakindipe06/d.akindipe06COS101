struct Laptop {
    brand:String,
    price:u64,
    quantity:u64,
}

impl Laptop {
    // Method to calculate total cost for a given quantity
    fn total_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    // Define laptops with their price and quantity
    let hp = Laptop { brand: "HP".to_string(), price: 650_000, quantity: 10 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755_000, quantity: 6 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), price: 550_000, quantity: 10 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850_000, quantity: 4 };

    // Calculate total cost for purchasing 3 of each brand
    let total_cost = hp.total_cost(3) + ibm.total_cost(3) + toshiba.total_cost(3) + dell.total_cost(3);

    // Output the total cost
    println!("The total cost for purchasing 3 laptops from each brand is: {}", total_cost);
}