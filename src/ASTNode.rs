enum ASTNode {
    Program(Vec<ASTNode>),
    Assignment(String, Box<ASTNode>),
    BinaryOp(String, Box<ASTNode>, Box<ASTNode>),
    UnaryOp(String, Box<ASTNode>),
    Identifier(String),
    Literal(f64),
}

pub fn parse_program(tokens: &mut Vec<Token>) -> ASTNode {
    let mut statements = Vec::new();
    while !tokens.is_empty() {
        statements.push(parse_statement(tokens));
    }
    ASTNode::Program(statements)
}

pub fn parse_statement(tokens: &mut Vec<Token>) -> ASTNode {
    match tokens.first() {
        Some(Token::Identifier(_)) => parse_assignment(tokens),
        _ => panic!("Invalid statement"),
    }
}

pub fn parse_assignment(tokens: &mut Vec<Token>) -> ASTNode {
    let identifier = match tokens.remove(0) {
        Token::Identifier(name) => name,
        _ => panic!("Expected identifier"),
    };
    if tokens.remove(0) != Token::Equals {
        panic!("Expected equals");
    }
    let expression = parse_expression(tokens);
    ASTNode::Assignment(identifier, Box::new(expression))
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> ASTNode {
    // ... implementation for parsing expressions ...
}
