pub fn run() {
    // Print to console
    println!("Hello from the printrs file");

    // Basic formatting
    println!("{} is from {}", "Tejaswi", "Chandausi");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Tejaswi", "Chandausi", "Code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Tejaswi",
        activity = "Call of duty"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 85, 134, 130);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic maths
    println!("10 + 10 = {}", 10 + 10);
}
