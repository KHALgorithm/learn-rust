//! Temperature Converter
//!
//! A simple command-line application that converts temperatures between Celsius and Fahrenheit.
//! The program provides a menu-driven interface for users to select the conversion type
//! and input the temperature value.

use std::io;

/// Main entry point of the program.
///
/// Displays a menu for temperature conversion options and handles user input
/// to determine which conversion to perform.
fn main() {
    // Display program title and menu options
    println!("Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Please select an option (1 or 2):");

    // Read user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    // Parse and validate user input
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    // Execute the selected conversion
    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("Invalid choice. Please select 1 or 2.");
    }
}

/// Converts a temperature from Celsius to Fahrenheit.
///
/// # Examples
/// ```
/// celsius_to_fahrenheit(); // Prompts for input: 0
/// // Output: 0.00 Celsius is 32.00 Fahrenheit
/// ```
///
/// # Formula
/// Fahrenheit = (Celsius × 9/5) + 32
fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read input");

    // Parse and validate the temperature input
    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Perform the conversion
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{:.2} Celsius is {:.2} Fahrenheit", celsius, fahrenheit);
}

/// Converts a temperature from Fahrenheit to Celsius.
///
/// # Examples
/// ```
/// fahrenheit_to_celsius(); // Prompts for input: 32
/// // Output: 32.00 Fahrenheit is 0.00 Celsius
/// ```
///
/// # Formula
/// Celsius = (Fahrenheit - 32) × 5/9
fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read input");

    // Parse and validate the temperature input
    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Perform the conversion
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
}
