// Project: rust-lexer
// Author: Aldo A. Avalos, Jr.
// Purpose: A simple lexer written in Rust
// Compiler: rustc 1.51.0 (2fd73fabe 2021-03-23)
// Date: 2023-26-04

// Notes: This is the main file for the rust-lexer project. 
// It contains the main function, which is the entry point for the program.
// The main function gets the input file from the command line arguments, 
// opens the input file, reads the input file into a string, 
// and calls the lexer function from the lexer module to lex the input string. 
// The main function then prints the tokens returned by the lexer function.

use std::env; // Import the env module to get the command line arguments
use std::fs::File; // Import the File struct to open files
use std::io::Read; // Import the Read trait to read files

mod lexer; // Import the lexer module

fn main() {
    let args: Vec<String> = env::args().collect(); // Get the command line arguments

    // Check that the user has provided an input file
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]); // Error message if no input file is provided
        return; // Exit the program if no input file is provided
    }

    // Lexical analysis phase
    let input_file: &String = &args[1]; // Get the input file
    let mut file: File = File::open(input_file).unwrap(); // Open the input file
    let mut input: String = String::new(); // Create a string to store the input
    file.read_to_string(&mut input).unwrap(); // Read the input file into the string
    let tokens: Vec<String> = lexer::lexer(&input); // Lex the input string
    println!( "{:?}", tokens); // Print the tokens returned by the lexer function
}

