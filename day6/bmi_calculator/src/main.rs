// BMI = weight (kg) / height (m)^2
// BMI < 18.5: Underweight
// BMI 18.5 - 24.9: Normal weight
// BMI 25.0 - 29.9: Overweight
// BMI >= 30.0: Obesity

// Import the standard input/output library for reading user input
use std::io;

/// The main function is the entry point of the program
fn main() {
    // Print the title of the program
    println!("BMI Calculator");
    // Ask the user to enter their weight in kilograms
    println!("Enter your weight in kg:");

    // Call the function to get user input and parse it as f64 (floating point number)
    // Use match to handle the result: Some(w) if successful, None if not
    let weight = match get_input_as_f64() {
        Some(w) => w, // If parsing is successful, assign the value to weight
        None => {
            // If parsing fails, print an error message and exit the program
            println!("Invalid weight input. Please enter a valid number.");
            return;
        }
    };

    // Ask the user to enter their height in meters
    println!("Enter your height in meters:");
    // Call the function again to get and parse the height
    let height = match get_input_as_f64() {
        // NOTE: There is a typo here: should be Some(h), not some(h)
        Some(h) => h, // If parsing is successful, assign the value to height
        None => {
            // If parsing fails, print an error message and exit the program
            println!("Invalid height input. Please enter a valid number.");
            return;
        }
    };

    // Check if the height is zero to avoid division by zero
    if height == 0.0 {
        println!("Height cannot be zero. Please enter a valid height.");
        return;
    }
    // Calculate BMI using the provided weight and height
    let bmi = calculate_bmi(weight, height);
    // Print the calculated BMI, formatted to 2 decimal places
    println!("Your BMI is: {:.2}", bmi);

    // Determine the BMI category (Underweight, Normal, etc.)
    let category = categorize_bmi(bmi);
    // Print the BMI category
    println!("You are classified as: {}", category);
}

/// Reads a line from standard input and tries to parse it as f64 (floating point number)
/// Returns Some(f64) if successful, or None if parsing fails
fn get_input_as_f64() -> Option<f64> {
    // Create a new, empty String to store user input
    let mut input = String::new();
    // Read a line from standard input and store it in 'input'
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // Panic if reading fails

    // Trim whitespace and try to parse the input as f64
    match input.trim().parse::<f64>() {
        Ok(value) => Some(value), // If parsing succeeds, return the value
        Err(_) => None,           // If parsing fails, return None
    }
}

/// Calculates the BMI given weight (kg) and height (m)
/// Returns the BMI as f64
fn calculate_bmi(weight: f64, height: f64) -> f64 {
    // BMI formula: weight divided by height squared
    weight / (height * height)
}

/// Categorizes the BMI value into a string label
/// Returns a string slice (&'static str) representing the category
fn categorize_bmi(bmi: f64) -> &'static str {
    // If BMI is less than 18.5, return "Underweight"
    if bmi < 18.5 {
        "Underweight"
    // If BMI is less than 25.0, return "Normal weight"
    } else if bmi < 25.0 {
        "Normal weight"
    // If BMI is less than 30.0, return "Overweight"
    } else if bmi < 30.0 {
        "Overweight"
    // Otherwise, return "Obesity"
    } else {
        "Obesity"
    }
}




// // Option<T> is used when a value might be present or not.
// // - Some(value): There is a value.
// // - None: There is no value.
// let maybe_number: Option<i32> = Some(42); // There is a number
// let no_number: Option<i32> = None;        // There is no number

// // Use Option when absence of a value is expected and not an error.
// // Example: Searching for an item in a list might return Some(item) or None if not found.

// // Result<T, E> is used for operations that can succeed or fail.
// // - Ok(value): The operation succeeded, and here is the value.
// // - Err(error): The operation failed, and here is the error.
// let success: Result<i32, &str> = Ok(10);      // Operation succeeded
// let failure: Result<i32, &str> = Err("fail"); // Operation failed

// // Use Result when you want to know if an operation succeeded or failed, and why.
// // Example: Parsing a string to a number can succeed (Ok(number)) or fail (Err(error)).

// fn parse_number(input: &str) -> Option<i32> {
//     // Try to parse the input as a number.
//     // If parsing succeeds, return Some(number).
//     // If parsing fails, return None.
//     input.parse::<i32>().ok()
//     // .ok() converts Result to Option: Ok(v) -> Some(v), Err(_) -> None
// }

// fn parse_number_with_error(input: &str) -> Result<i32, std::num::ParseIntError> {
//     // Try to parse the input as a number.
//     // If parsing succeeds, return Ok(number).
//     // If parsing fails, return Err(error).
//     input.parse::<i32>()
// }

// // Example usage:
// let opt = parse_number("123"); // Some(123)
// let opt_fail = parse_number("abc"); // None

// let res = parse_number_with_error("123"); // Ok(123)
// let res_fail = parse_number_with_error("abc"); // Err(ParseIntError)

// // Summary:
// // - Use Option<T> (Some/None) when a value might be missing, but it's not an error.
// // - Use Result<T, E> (Ok/Err) when an operation can fail and you want to know the reason.
