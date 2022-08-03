pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Riku", "NYC");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Riku", "NYC", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Riku", activity = "poker");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10 );

    // Placeolder for debug trait - Also called a tuple
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}