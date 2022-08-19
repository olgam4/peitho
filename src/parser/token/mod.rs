#[derive(new, Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // -- Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // COMMA,
    // DOT,
    // MINUS,
    Plus,
    // SEMICOLON,
    // SLASH,
    // STAR,

    // -- One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // -- Literals.
    // IDENTIFIER,
    String,
    Number,

    // -- Keywords.
    // AND,
    // CLASS,
    Else,
    // FALSE,
    // FUN,
    // FOR,
    If,
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

