use std::process::Command;

fn main() {
    // Specify the name of the software you want to check
    let software_name = "Steam";

    // Run the 'winget' command to list installed software
    let output = Command::new("winget")
        .arg("list")
        .output()
        .expect("Failed to execute winget");

    // Convert the output to a string for searching
    let installed_software = String::from_utf8_lossy(&output.stdout);
    println!("{}", installed_software);

    // Check if the desired software is in the list
    if installed_software.contains(software_name) {
        println!("{} is installed!", software_name);
    } else {
        println!("{} is not installed.", software_name);
    }
}
