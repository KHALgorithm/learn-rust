# Day 1: Getting Started with Rust

## Introduction to Rust
Rust is a modern systems programming language that focuses on safety, speed, and concurrency. It's designed to prevent common programming errors like null pointer dereferencing and data races.

## Setting Up Rust
1. Install Rust using rustup (the official Rust installer):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Verify your installation:
```bash
rustc --version  # Shows Rust compiler version
cargo --version  # Shows Cargo package manager version
```

## Essential Cargo Commands
Cargo is Rust's package manager and build system. Here are the most important commands:

```bash
# Create a new Rust project
cargo new hello_rust

# Run your project
cargo run

# Build your project (creates executable in target/debug)
cargo build

# Check your code for errors without producing an executable
cargo check

# Run tests in your project
cargo test

# Generate documentation
cargo doc

# Build optimized release version (creates executable in target/release)
cargo build --release
```

## Your First Rust Program
Let's create a simple "Hello, World!" program and explore different ways to print in Rust:

```rust
// main.rs
fn main() {
    // Basic print (no newline)
    print!("Hello, ");

    // Print with newline
    println!("World!");

    // Print with formatting
    let name = "Rust";
    println!("Hello, {}!", name);

    // Print with multiple values
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);

    // Print with debug formatting
    println!("Debug format: {:?}", (x, y));

    // Print with pretty debug formatting
    println!("Pretty debug format: {:#?}", (x, y));
}
```

## Understanding the Code
1. `fn main()` - The entry point of every Rust program
2. `print!` vs `println!` - The `!` indicates these are macros, not functions
   - `print!` - Prints without a newline
   - `println!` - Prints with a newline
3. Formatting placeholders:
   - `{}` - Default formatting
   - `{:?}` - Debug formatting
   - `{:#?}` - Pretty debug formatting

## Project Structure
When you create a new Rust project with `cargo new`, you get:
```
hello_rust/
├── Cargo.toml    # Project configuration and dependencies
└── src/
    └── main.rs   # Source code
```

## Basic Rust Concepts
1. **Variables and Mutability**
```rust
let x = 5;           // Immutable by default
let mut y = 10;      // Mutable variable
```

2. **Basic Types**
```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'A';
let string: String = String::from("Hello");
```

3. **Comments**
```rust
// This is a single-line comment
/* This is a
   multi-line comment */
```

## Functions and Macros in Rust

### Functions
Functions in Rust are declared using the `fn` keyword. Here's a comprehensive look at functions:

```rust
// Basic function
fn greet() {
    println!("Hello!");
}

// Function with parameters
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon means this is the return value
}

// Function with multiple return values using tuples
fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x - y)
}

// Function with default parameters (using Option)
fn greet_with_title(name: &str, title: Option<&str>) {
    match title {
        Some(t) => println!("Hello, {} {}!", t, name),
        None => println!("Hello, {}!", name),
    }
}

// Function with generic type
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value is: {}", value);
}
```

### Macros
Macros in Rust are more powerful than functions and are identified by the `!` at the end of their name. They can:
- Take a variable number of arguments
- Generate code at compile time
- Transform code before compilation

#### Common Built-in Macros:

1. **Printing Macros**
```rust
// Basic printing
print!("Hello");           // Prints without newline
println!("Hello");         // Prints with newline
eprint!("Error");         // Prints to stderr without newline
eprintln!("Error");       // Prints to stderr with newline

// Formatting
format!("Hello, {}!", "Rust");  // Returns a String
```

2. **Assertion Macros**
```rust
assert!(1 + 1 == 2);                    // Basic assertion
assert_eq!(1 + 1, 2);                   // Equality assertion
assert_ne!(1 + 1, 3);                   // Inequality assertion
debug_assert!(1 + 1 == 2);              // Debug-only assertion
```

3. **Vector Macros**
```rust
let v = vec![1, 2, 3];                  // Creates a vector
let v = vec![0; 5];                     // Creates [0, 0, 0, 0, 0]
```

4. **Error Handling Macros**
```rust
panic!("Something went wrong!");        // Panics with message
unreachable!();                         // Indicates unreachable code
unimplemented!();                       // Indicates unimplemented code
```

#### Creating Custom Macros
Here's a simple example of a custom macro:

```rust
// Declarative macro
macro_rules! greet {
    () => {
        println!("Hello, World!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// Using the custom macro
fn main() {
    greet!();           // Prints: Hello, World!
    greet!("Rust");     // Prints: Hello, Rust!
}
```

### Key Differences Between Functions and Macros

1. **Syntax**
   - Functions: `fn name() {}`
   - Macros: `name!()`

2. **Arguments**
   - Functions: Fixed number of arguments, specific types
   - Macros: Variable number of arguments, can accept different types

3. **Compilation**
   - Functions: Compiled as is
   - Macros: Expanded at compile time into different code

4. **Use Cases**
   - Functions: For regular program logic
   - Macros: For code generation, metaprogramming, and reducing boilerplate

### Best Practices
1. Use functions for regular program logic
2. Use macros when you need:
   - Code generation
   - Variable number of arguments
   - To reduce repetitive code
   - To implement domain-specific languages (DSLs)

## Next Steps
- Learn about Rust's ownership system
- Explore control flow (if/else, loops)
- Study functions and modules
- Understand error handling

## Additional Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
