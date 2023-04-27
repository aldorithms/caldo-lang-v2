pub fn syntaxer(tokens: Vec<String>) {
    // Loop over the tokens to analyze syntax
    for token in tokens {
        match token {
            // If the token is a keyword
            "fn" => {
                // ... implementation for parsing functions ...
                for token in tokens {
                    
                }
            }
            // ... more keywords ...
            _ => {
                // ... implementation for parsing expressions ...
                panic!("Invalid token: {}", token);
            }
        }
    }
}