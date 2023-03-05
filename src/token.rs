pub fn str_to_keyword(word: &str) -> Option<Token> {
    match word {
        "let" => Some(Token::Let),
        "nil" => Some(Token::Nil),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub enum Token {
    // Literals
    Nil,
    Number { value: String },
    Identifier { value: String },
    String { value: String },

    // Keyword
    Let,

    // Grouping + Ops
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Percent,

    BinOp,
    Eq,
    OpenParen,
    CloseParen,
    EOF,
}

pub static LET_KEYWORD: &str = "let";
pub static NIL_KEYWORD: &str = "nil";
