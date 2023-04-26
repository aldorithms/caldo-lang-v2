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
                    // If the character is a parenthesis, brace, colon, or comma, push it to the vector
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

    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    tokens
}
