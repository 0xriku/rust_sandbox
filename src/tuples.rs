// Tuples group together values of different types, don't have to all be the same type like Arrays
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Riku", "NYC", 25);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}