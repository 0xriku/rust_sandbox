// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite Loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     // Must stop the loop otherwise iterates forever
    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop (FizzBuzz)
    // Loop through 0 to 100
    // If number is divisible by 3, return Fizz
    // If number if divisible by 5, return Buzz
    // If number is divisible by 3 and 5, return FizzBuzz

    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     // Inc
    //     count += 1;
    // }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}