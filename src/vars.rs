// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Riku";
    let mut age = 25;
    println!("My name is {} and I am {}", name, age);
    age = 26;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Riku", 25);
    println!("{} is {}", my_name, my_age);
}