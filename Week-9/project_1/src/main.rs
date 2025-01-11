use std::fs::File;
use std::io::Write;

fn main() {
    
    let data = [
        ("Lager", "33 Export, Desperados, Goldberg, Gulder, Heineken, Star"),
        ("Stout", "Legend, Turbo King, Williams"),
        ("Non-Alcoholic", "Maltina, Amstel Malta, Malta Gold, Fayrouz"),
    ];

    let mut file = File::create("drink_categories.csv").expect("Unable to create file");

    file.write_all(b"Category,Drinks\n").expect("Unable to write header");

    for i in 0..data.len() {
        let (category, drinks) = data[i]; 
        let line = format!("{},{}\n", category, drinks);
        file.write_all(line.as_bytes()).expect("Unable to write data");
    }

    println!("CSV file 'drink_categories.csv' created successfully!");
}