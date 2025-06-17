//! Word Counter Project
//! ===================
//!
//! This project demonstrates basic file I/O operations and string manipulation in Rust.
//! It reads a text file and provides various text analysis features:
//! - Word count
//! - Character count
//! - Line count
//! - Specific word frequency
//! - Case-insensitive word frequency
//!
//! Key Concepts Covered:
//! - Command-line argument handling
//! - File I/O operations
//! - Error handling with Result and Option
//! - String manipulation
//! - Pattern matching
//! - Collections (HashMap)
//! - Iterators
//! - Closures
//!
//! Usage:
//! ------
//! Basic usage:
//! cargo run <filename>
//!
//! Advanced usage:
//! cargo run <filename> --word <specific_word>
//! cargo run <filename> --stats
//!
//! Example:
//! cargo run test.txt --stats
//! cargo run test.txt --word "hello"

// Import required modules from the standard library
use std::env;      // For accessing command-line arguments
use std::fs::File; // For file operations
use std::io::Read; // For reading file contents
use std::collections::HashMap;

/// Structure to hold text statistics
struct TextStats {
    word_count: usize,
    char_count: usize,
    line_count: usize,
    word_frequency: HashMap<String, usize>,
}

impl TextStats {
    /// Create new TextStats instance
    fn new() -> Self {
        TextStats {
            word_count: 0,
            char_count: 0,
            line_count: 0,
            word_frequency: HashMap::new(),
        }
    }

    /// Calculate statistics from text content
    fn calculate(&mut self, contents: &str) {
        // Count characters
        self.char_count = contents.chars().count();

        // Count lines
        self.line_count = contents.lines().count();

        // Count words and build frequency map
        for word in contents.split_whitespace() {
            self.word_count += 1;
            *self.word_frequency.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }

    /// Print all statistics
    fn print_stats(&self) {
        println!("\nText Statistics:");
        println!("----------------");
        println!("Words: {}", self.word_count);
        println!("Characters: {}", self.char_count);
        println!("Lines: {}", self.line_count);
        println!("\nWord Frequency (top 5):");
        println!("----------------------");

        // Sort and print top 5 most frequent words
        let mut word_vec: Vec<(&String, &usize)> = self.word_frequency.iter().collect();
        word_vec.sort_by(|a, b| b.1.cmp(a.1));

        for (word, count) in word_vec.iter().take(5) {
            println!("'{}': {}", word, count);
        }
    }

    /// Print frequency of a specific word
    fn print_word_frequency(&self, target_word: &str) {
        let count = self.word_frequency.get(&target_word.to_lowercase()).unwrap_or(&0);
        println!("\nFrequency of '{}': {}", target_word, count);
    }
}

/// Main function that orchestrates the text analysis process
fn main() {
    // Step 1: Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Step 2: Validate input arguments
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [--stats] [--word <specific_word>]", args[0]);
        std::process::exit(1);
    }

    // Step 3: Get the file path from arguments
    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    // Step 4: Open and read the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    }

    // Step 5: Calculate statistics
    let mut stats = TextStats::new();
    stats.calculate(&contents);

    // Step 6: Process command line options
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--stats" => {
                stats.print_stats();
            }
            "--word" => {
                if i + 1 < args.len() {
                    stats.print_word_frequency(&args[i + 1]);
                    i += 1;
                } else {
                    eprintln!("Error: --word requires a word argument");
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
            }
        }
        i += 1;
    }

    // If no options provided, show basic word count
    if args.len() == 2 {
        println!("Word count: {}", stats.word_count);
    }
}

// for testing, create a text file named `test.txt` with some sample text

// show all statistics:
// cargo run test.txt --stats

// Count frequency of a specific word:
// cargo run test.txt --word "hello"

// Count words in the file without any options:
// cargo run test.txt
