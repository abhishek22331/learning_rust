use std::io;

pub fn sum() {
    let x = 5;
    let y = 6;
    println!("Sum from fun: {}", x + y);

    let mut user_input = String::new();

    println!("Please enter something:");

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

   
    let trimmed_input = user_input.trim();

    if trimmed_input.is_empty() {
        println!("Error: Input cannot be empty.");
    } else if trimmed_input.chars().all(|ch| ch.is_alphabetic()) {
        // Check if all characters are alphabetic
        println!("You entered a string: {}", trimmed_input);
    } else {
        // Try parsing the input as an i32
        match trimmed_input.parse::<i32>() {
            Ok(number) => {
                println!("You entered the number: {}", number);
            }
            Err(_) => {
                println!("Error: Please enter a valid number or alphabetic string.");
            }
        }
    }
}
