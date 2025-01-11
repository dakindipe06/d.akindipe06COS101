use std::fs::File;
use std::io::Write;

fn main() {
   
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let header = format!(
        "{:<5} {:<30} {:<5} {:<20} {:<5} {:<15}\n",
        "S/N", "NAME OF COMMISSIONER", "S/N", "MINISTRY", "S/N", "GEOPOLITICAL ZONE"
    );

    let separator = "-".repeat(90);

    let mut file = File::create("merged_table.txt").expect("Unable to create file");

    file.write_all(header.as_bytes()).expect("Failed to write header");
    file.write_all(format!("{}\n", separator).as_bytes())
        .expect("Failed to write separator");

    for i in 0..commissioners.len() {
        let row = format!(
            "{:<5} {:<30} {:<5} {:<20} {:<5} {:<15}\n",
            i + 1,
            commissioners[i],
            i + 1,
            ministries[i],
            i + 1,
            geopolitical_zones[i],
        );
        file.write_all(row.as_bytes()).expect("Failed to write row");
    }

    println!("\nData successfully written to 'merged_table.txt' in table format.");
}