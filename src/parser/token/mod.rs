#[derive(new, Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    // LEFTPAREN,
    // RIGHTPAREN,
    // LEFTBRACE,
    // RIGHTBRACE,
    // COMMA,
    // DOT,
    // MINUS,
    PLUS,
    // SEMICOLON,
    // SLASH,
    // STAR,

    // One or two character tokens.
    // BANG,
    // BANGEQUAL,
    EQUAL,
    // EQUALEQUAL,
    // GREATER,
    // GREATEREQUAL,
    // LESS,
    // LESSEQUAL,

    // Literals.
    // IDENTIFIER,
    STRING,
    // NUMBER,

    // Keywords.
    // AND,
    // CLASS,
    // ELSE,
    // FALSE,
    // FUN,
    // FOR,
    // IF,
    // NIL,
    // OR,
    PRINT,
    // RETURN,
    // SUPER,
    // THIS,
    // TRUE,
    // VAR,
    // WHILE,

    // EOF,
}

