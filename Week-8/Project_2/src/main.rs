fn main() {
    // A vector of tuples containing names and years of experience
    let developers = vec![
        ("Tsunade", 8),
        ("Dante", 10),
        ("Timi", 23),
        ("Diane", 10),
    ];

    // Initialize variables to track the developer with the highest experience
    let mut max_experience = 0;
    let mut top_developer = "";

    // Iterate through the vector to find the developer with the highest experience
    for (name, experience) in developers {
        if experience > max_experience {
            max_experience = experience;
            top_developer = name;
        }
    }

    // Print the result
    println!(
        "The developer with the highest experience is {:?} with {:?} years.",
        top_developer, max_experience
    );
}