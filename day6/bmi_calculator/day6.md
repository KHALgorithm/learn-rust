# Rust Fundamentals: Option, Result, and Return Types

## Understanding the Big Picture

In Rust, we often need to handle two common situations: values that might not exist, and operations that might fail. Rust provides elegant solutions for both through special types called `Option` and `Result`. Additionally, understanding what functions can return helps you write better code.

## Part 1: When Values Might Not Exist - The Option Type

Think of `Option` like a box that might contain something or might be empty. This is incredibly useful in programming because sometimes we search for something and don't find it, or we try to get the first item from an empty list.

### The Option Enum

The `Option<T>` type has two possibilities:
- `Some(value)` - the box contains a value
- `None` - the box is empty

Here's how this works in practice:

```rust
// A function that might find a number in a list
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // Found an even number, return it wrapped in Some
        }
    }
    None // Didn't find any even numbers, return None
}

// Using the function
let numbers = [1, 3, 6, 9];
match find_first_even(&numbers) {
    Some(even_num) => println!("Found even number: {}", even_num),
    None => println!("No even numbers found"),
}
```

## Part 2: When Operations Might Fail - The Result Type

While `Option` handles "maybe there, maybe not," `Result` handles "succeeded or failed, and if it failed, here's why." Think of `Result` as a more informative version of a simple success/failure indicator.

### The Result Enum

The `Result<T, E>` type has two possibilities:
- `Ok(value)` - the operation succeeded and here's the result
- `Err(error)` - the operation failed and here's what went wrong

Here's a practical example:

```rust
// A function that tries to divide two numbers
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string()) // Operation failed
    } else {
        Ok(a / b) // Operation succeeded
    }
}

// Using the function
match safe_divide(10.0, 2.0) {
    Ok(result) => println!("Division result: {}", result),
    Err(error_msg) => println!("Error: {}", error_msg),
}
```

## Part 3: Understanding Function Return Types

Functions in Rust must declare what type of value they return. This helps both you and the compiler understand what to expect. Let's explore the most common return types you'll encounter.

### Simple Value Returns

When your function calculates or produces a single value:

```rust
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height // Returns a single floating-point number
}

fn is_adult(age: u32) -> bool {
    age >= 18 // Returns true or false
}
```

### String Returns - Two Important Flavors

Understanding the difference between `String` and `&str` is crucial:

```rust
// Returns an owned, growable string (you can modify it)
fn create_greeting(name: &str) -> String {
    format!("Welcome, {}!", name) // Creates a new string
}

// Returns a reference to existing string data (read-only)
fn get_error_message() -> &'static str {
    "Something went wrong" // Points to string literal in program memory
}
```

The `&'static str` type means "a string slice that lives for the entire program duration." This is perfect for constant messages and string literals.

### Multiple Values with Tuples

Sometimes you need to return several related values:

```rust
fn analyze_text(text: &str) -> (usize, usize, bool) {
    let word_count = text.split_whitespace().count();
    let char_count = text.len();
    let has_exclamation = text.contains('!');

    (word_count, char_count, has_exclamation) // Returns all three values
}

// Using the returned tuple
let (words, chars, excited) = analyze_text("Hello world!");
```

### The Unit Type - When Nothing Is Returned

Some functions do work but don't need to return a value:

```rust
fn log_message(msg: &str) {
    println!("[LOG] {}", msg);
    // This function returns (), the unit type (like void in other languages)
}
```

## Putting It All Together

Here's how these concepts work together in a realistic example:

```rust
fn process_user_input(input: &str) -> Result<Option<f64>, String> {
    if input.trim().is_empty() {
        return Ok(None); // Empty input is valid but gives no number
    }

    match input.trim().parse::<f64>() {
        Ok(number) => Ok(Some(number)), // Successfully parsed a number
        Err(_) => Err("Invalid number format".to_string()), // Parse failed
    }
}

// This function returns a Result that contains an Option!
// - Ok(Some(number)) = successfully parsed a number
// - Ok(None) = input was empty (valid but no number)
// - Err(message) = parsing failed
```

## Key Takeaways

Understanding these types helps you write safer, more expressive code. Use `Option` when something might not exist, use `Result` when an operation might fail, and choose your return types based on what your function actually produces. The Rust compiler will help guide you toward handling all the possible cases, making your programs more robust and reliable.

## Quick Reference

- `Option<T>`: Use for "maybe has a value"
  - `Some(value)` when there is a value
  - `None` when there isn't

- `Result<T, E>`: Use for "might succeed or fail"
  - `Ok(value)` when operation succeeds
  - `Err(error)` when operation fails

- Common return types:
  - `i32`, `f64`, `bool` for simple values
  - `String` for owned text you can modify
  - `&str` or `&'static str` for borrowed text
  - `()` when you don't return anything
  - `(T1, T2, ...)` for multiple values
