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
use std::rc::Rc; // Import the Rc struct to create reference counted pointers
use std::cell::RefCell; // Import the RefCell struct to create mutable references

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
    let tokens: Vec<String> = lexer(&input); // Lex the input string

    println!( "{:?}", tokens); // Print the tokens returned by the lexer function

    // Syntax analysis phase
    

    // Semantic analysis phase

} // 

pub fn lexer(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new(); // Create a vector to store the tokens
    let mut buffer: String = String::new(); // Create a buffer to store the current token
    let mut chars: std::iter::Peekable<std::str::Chars> = input.chars().peekable(); // Create an iterator over the characters in the input string

    // Loop over the characters in the input string
    while let Some(&c) = chars.peek() {
        // Match the current character
        match c {
            // If the character is a letter or underscore, it must be an identifier
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                buffer.push(c); // Add the current character to the buffer
                chars.next(); // Advance the iterator
            }
            // If the character is a space, newline, or tab, we have reached the end of a token
            ' ' | '\n' | '\r' => {
                // If the buffer is not empty, push it to the vector
                if !buffer.is_empty() {
                    tokens.push(buffer.clone()); // Push the buffer to the vector
                    buffer.clear(); // Clear the buffer
                }
                chars.next(); // Advance the iterator
            }
            // If the character is a parenthesis, brace, colon, or comma, we have reached the end of a token
            _ => {
                // If the buffer is not empty, push it to the vector
                if !buffer.is_empty() {
                    tokens.push(buffer.clone()); // Push the buffer to the vector
                    buffer.clear(); // Clear the buffer
                }
                // Match the current character
                match c {
                    // If the character is an operator
                    '(' | ')' | '{' | '}' | ':' | ',' => {
                        tokens.push(c.to_string()); // Push the character to the vector
                        chars.next(); // Advance the iterator
                    }
                    // If the character is a slash, we may have a comment
                    _ => {
                        panic!("Invalid character: {}", c); // Panic if the character is invalid.
                    }
                }
            }
        }
    }
    // If the buffer is not empty, push it to the vector
    if !buffer.is_empty() {
        tokens.push(buffer); // Push the buffer to the vector
    }
    tokens // Return the vector of tokens
} // End of function lexer

fn syntaxer(tokens: &Vec<String>) -> AST {
    let mut _ast: AST = AST::new(); // Create an abstract syntax tree
    let mut _current: Node = Node::new(); // Create a node to store the current node

    // Loop over the tokens
    for token in *tokens {
        // Match the current token
        match token.as_str() {
            // If the token is a left parenthesis
            "(" => {
                _ast.add_node(token); // Add the token to the abstract syntax tree
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.children[0]); // Set the current node to the first child
            }
            // If the token is a right parenthesis
            ")" => {
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.parent); // Set the current node to the parent
            }
            // If the token is a left brace
            "{" => {
                _ast.add_node(token); // Add the token to the abstract syntax tree
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.children[0]); // Set the current node to the first child
            }
            // If the token is a right brace
            "}" => {
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.parent); // Set the current node to the parent
            }
            // If the token is a colon
            ":" => {
                _ast.add_node(token); // Add the token to the abstract syntax tree
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.children[0]); // Set the current node to the first child
            }
            // If the token is a comma
            "," => {
                _ast.add_node(token); // Add the token to the abstract syntax tree
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.children[0]); // Set the current node to the first child
            }
            // If the token is an identifier
            _ => {
                _ast.add_node(token); // Add the token to the abstract syntax tree
                _current = _ast.get_current(); // Get the current node
                _ast.set_current(_current.children[0]); // Set the current node to the first child
            }
        }
    }
    _ast // Return the abstract syntax tree
}

// Abstract Syntax Tree structure
pub struct AST {
    pub root: Node,
    pub current: Node,
}

impl AST {
    pub fn new() -> AST {
        AST {
            root: Node::new(), 
            current: Node::new(),
        }
    }
    pub fn add_node(&mut self, value: String) {
        let mut node: Node = Node::new();
        node.value = value;
        self.current.children.push(node);
    }

    pub fn add_child(&mut self, value: String) {
        let mut node: Node = Node::new();
        node.value = value;
        self.current.children.push(node);
    }

    pub fn add_sibling(&mut self, value: String) {
        let mut node: Node = Node::new();
        node.value = value;
        self.current.children.push(node);
    }

    pub fn add_parent(&mut self, value: String) {
        let mut node: Node = Node::new();
        node.value = value;
        self.current.children.push(node);
    }

    pub fn get_current(self) -> Node {
        self.current
    }

    pub fn set_current(&mut self, node: Node) {
        self.current = node;
    }

    pub fn get_root(self) -> Node {
        self.root
    }

    pub fn set_root(&mut self, node: Node) {
        self.root = node;
    }

    pub fn print(&mut self) {
        println!("Root: {}", self.root.value);
        println!("Current: {}", self.current.value);
    }

}

// Node structure
pub struct Node {
    pub value: String,
    pub children: Vec<Node>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            value: String::new(),
            children: Vec::new(),
            parent: None,
        }
    }
}