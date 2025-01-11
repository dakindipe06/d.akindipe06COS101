use std::fs;

fn main() {
    fs::remove_file("Data.txt").expect("could not remove file");
    println!("File is removed");
}