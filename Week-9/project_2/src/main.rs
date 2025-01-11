use std::fs::File;
use std::io::Write;

fn main() {
    // Define student details as a vector of tuples
    let students = vec![
        ("Student Name", "Matric. Number", "Department", "Level"), // Header
        ("Oluchi Mordi", "ACC1021111", "Accounting", "300"),
        ("Adams Aliyu", "ECO101101", "Economics", "100"),
        ("Shania Bolade", "CSC103828", "Computer", "200"),
        ("Adekunle Gold", "EEE110202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE102001", "Mechanical", "100"),
    ];

    // Display the table header and content in the terminal
    println!("\n{:^50}", "PAU SMIS"); // Centered Title
    println!();
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.0, student.1, student.2, student.3
        );
    }

    // Write the table content to a text file
    let mut file = File::create("students.txt").expect("Could not create file");

    // Add the title to the file
    file.write_all(b"PAU SMIS\n\n").expect("Failed to write title to file");

    // Write the table rows (header + content) to the file
    for student in &students {
        let line = format!(
            "{:<20} {:<15} {:<15} {:<5}\n",
            student.0, student.1, student.2, student.3
        );
        file.write_all(line.as_bytes()).expect("Failed to write data to file");
    }

    println!("\nData successfully written to 'students.txt' in table format.");
}