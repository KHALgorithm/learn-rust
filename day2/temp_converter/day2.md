# Day 2 - Temperature Converter Project

## Lessons Learned

### 1. User Input Handling
- Learned how to use `std::io` for handling user input
- Understanding of `read_line()` method and its usage with mutable string references
- Practice with error handling using `expect()` for basic error cases

### 2. Type Conversion and Parsing
- Converting string input to numeric types using `parse()`
- Using the `match` expression for safe type conversion
- Handling parsing errors with pattern matching
- Working with different numeric types (`u32` for menu choice, `f64` for temperature)

### 3. Numeric Types in Rust
- Integer Types:
  - Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
  - Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
  - Default integer type is `i32`
  - `isize` and `usize` are architecture-dependent (32 or 64 bits)
- Floating-Point Types:
  - `f32`: 32-bit floating point (single precision)
  - `f64`: 64-bit floating point (double precision)
  - Default floating-point type is `f64`
- Common Use Cases:
  - `u32`: For positive whole numbers (like menu choices)
  - `i32`: For general integer calculations
  - `f64`: For decimal numbers (like temperature values)
  - `usize`: For array indexing and sizes

### 4. Control Flow
- Implementation of menu-driven program structure
- Using `if-else` statements for program flow control
- Early returns for error cases
- Function organization and modular code structure

### 5. String Manipulation
- Using `trim()` to remove whitespace from user input
- String formatting with `println!` macro
- Formatting floating-point numbers using `{:.2}` for 2 decimal places

### 6. Mathematical Operations
- Implementation of temperature conversion formulas:
  - Celsius to Fahrenheit: `(celsius * 9.0 / 5.0) + 32.0`
  - Fahrenheit to Celsius: `(fahrenheit - 32.0) * 5.0 / 9.0`
- Working with floating-point arithmetic in Rust

### 7. Error Handling
- Basic error handling for invalid user input
- Using pattern matching to handle parsing errors
- Providing user-friendly error messages

### 8. Code Organization
- Breaking down functionality into separate functions
- Clear function naming conventions
- Maintaining clean and readable code structure

### 9. Best Practices
- Using meaningful variable names
- Implementing input validation
- Providing clear user prompts and instructions
- Formatting output for better readability

### 10. Documentation and Comments Best Practices
- Documentation Comments:
  - Use `///` for documenting items (functions, structs, etc.)
  - Use `//!` for module-level documentation
  - Write documentation in Markdown format
  - Include examples in documentation using `# Examples` section
- Code Comments:
  - Use `//` for single-line comments
  - Use `/* */` for multi-line comments
  - Comment complex logic or non-obvious code
  - Keep comments up-to-date with code changes
- Best Practices:
  - Document public APIs thoroughly
  - Include parameter descriptions
  - Document return values
  - Add examples for complex functions
  - Keep comments concise and meaningful
  - Use comments to explain "why" not "what"
  - Document error cases and edge conditions
