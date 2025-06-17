use std::io;  // Import the input/output library

fn main() {
    // Display welcome message and instructions
    println!("Simple Calculator");
    println!("Available operations: + - * /");
    println!("Enter your expression (e.g., 3 + 4):");

    // Create a mutable string to store user input
    let mut input = String::new();
    // Read user input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split the input into tokens (numbers and operator)
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    // Validate input format
    if tokens.len() != 3 {
        println!("Invalid input. Please enter an expression in the format: number1 operator number2");
        return;
    }

    // Extract and validate the operator
    let operator = tokens[1];
    if !["+", "-", "*", "/"].contains(&operator) {
        println!("Invalid operator: {}", operator);
        return;
    }

    // Parse first number with error handling
    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", tokens[0]);
            return;
        }
    };

    // Parse second number with error handling
    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", tokens[2]);
            return;
        }
    };

    // Perform the calculation based on the operator
    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Unknown operator: {}", operator);
            return;
        }
    };

    // Display the result with 2 decimal places
    println!("Result: {:.2}", result);
}

// Arithmetic operation functions
fn add(a: f64, b: f64) -> f64 {
    // No semicolon, so this is automatically returned
    a + b // or return a+b;
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b // or return a-b;
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b // or return a*b;
}

fn divide(a: f64, b: f64) -> f64 {
    // Check for division by zero
    if b == 0.0 {
        panic!("Division by zero is not allowed.");
    }
    a / b // or return a/b;
}
