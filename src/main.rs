use std::env;
use std::fs::File;
use std::io::Read;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect(); // Get the command line arguments

    // Check that the user has provided an input file
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]); // Error message if no input file is provided
        return;
    }

    let input_file: &String = &args[1]; // Get the input file
    let mut file: File = File::open(input_file).unwrap(); // Open the input file
    let mut input: String = String::new(); // Create a string to store the input
    file.read_to_string(&mut input).unwrap(); // Read the input file into the string
    let tokens: Vec<String> = lexer::lexer(&input); // Lex the input string
    println!( "{:?}", tokens); // Print the tokens
}

