#[derive(new, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // -- Single-character tokens.
    LeftParen,
    RightParen,
    // LEFTBRACE,
    // RIGHTBRACE,
    // COMMA,
    // DOT,
    // MINUS,
    Plus,
    // SEMICOLON,
    // SLASH,
    // STAR,

    // -- One or two character tokens.
    // BANG,
    // BANGEQUAL,
    Equal,
    EqualEqual,
    // GREATER,
    // GREATEREQUAL,
    // LESS,
    // LESSEQUAL,

    // -- Literals.
    // IDENTIFIER,
    String,
    Number,

    // -- Keywords.
    // AND,
    // CLASS,
    // ELSE,
    // FALSE,
    // FUN,
    // FOR,
    // IF,
    // NIL,
    // OR,
    Print,
    // RETURN,
    // SUPER,
    // THIS,
    // TRUE,
    // VAR,
    // WHILE,

    // EOF,
    EOL,
}

