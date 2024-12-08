fn main() {
    // Define the data as a vector of tuples
    let public_service_roles = vec![
        ("APS 1-2", "Intern", "", "Paralegal", "Placement"),
        ("APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"),
        ("EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", "CEO", "Dean", "Partner", "Principal"),
    ];

    // Example staff details
    let staff_role = "Lawyer";
    let staff_experience = 6;

    // Call the function to determine the APS level
    if let Some(aps_level) = find_aps_level(&public_service_roles, staff_role, staff_experience) {
        println!("The staff belongs to the position: {}", aps_level);
    } else {
        println!("No matching APS level found for the given details.");
    }
}

// Function to determine APS level
fn find_aps_level(
    roles: &Vec<(&str, &str, &str, &str, &str)>,
    role: &str,
    experience: usize,
) -> Option<String> {
    for (level, office_admin, academic, lawyer, teacher) in roles.iter() {
        let role_match = match role {
            "Office Administrator" => office_admin,
            "Academic" => academic,
            "Lawyer" => lawyer,
            "Teacher" => teacher,
            _ => "",
        };

        // Check if experience matches the range for APS levels
        let experience_range = match *level {
            "APS 1-2" => 1..=2,
            "APS 3-5" => 3..=5,
            "APS 5-8" => 5..=8,
            "EL1 8-10" => 8..=10,
            "EL2 10-13" => 10..=13,
            _ => 0..=0, // Default case
        };

        if role_match == role && experience_range.contains(&experience) {
            return Some(level.to_string());
        }
    }

    None
}