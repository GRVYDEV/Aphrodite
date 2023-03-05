use regex::RegexSet;

#[derive(Clone, Debug)]
pub enum TokenType {
    // Literals
    Nil,
    Number,
    Identifier,
    String,

    // Keyword
    Let,

    // Grouping + Ops
    BinOp,
    Eq,
    OpenParen,
    CloseParen,
    EOF,
}

static LET_KEYWORD: &str = "let";
static NIL_KEYWORD: &str = "nil";

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

macro_rules! token {
    ($a:expr, $b:expr) => {
        Token {
            value: $a,
            token_type: $b,
        }
    };
}

#[derive(Debug, Clone)]
pub struct SyntaxError;

pub struct Tokenizer {
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { tokens: vec![] }
    }

    pub fn tokenize(&mut self, src: &str) -> Result<Vec<Token>, SyntaxError> {
        let mut src = src.chars().peekable();

        while let Some(tkn) = src.next() {
            match tkn {
                '(' => self
                    .tokens
                    .push(token!(tkn.to_string(), TokenType::OpenParen)),
                ')' => self
                    .tokens
                    .push(token!(tkn.to_string(), TokenType::CloseParen)),
                '+' | '*' | '/' | '%' => {
                    self.tokens.push(token!(tkn.to_string(), TokenType::BinOp))
                }
                '=' => self.tokens.push(token!(tkn.to_string(), TokenType::Eq)),
                ' ' | '\n' | '\t' => {}
                '"' => {
                    let mut chars: Vec<char> = vec![];
                    loop {
                        let chr = src.next();
                        match chr {
                            Some(chr) => {
                                if chr == '"' {
                                    break;
                                }
                                chars.push(chr)
                            }
                            None => {
                                println!("string did not close before EOF");
                                return Err(SyntaxError);
                            }
                        }
                    }
                    self.tokens.push(token!(
                        chars.into_iter().collect::<String>(),
                        TokenType::String
                    ))
                }
                _ => {
                    if tkn.is_numeric() {
                        let mut chars: Vec<char> = vec![tkn];
                        loop {
                            let chr = src.peek();
                            match chr {
                                Some(chr) => {
                                    if !chr.is_numeric() {
                                        break;
                                    }
                                    let t = src.next();
                                    chars.push(t.unwrap());
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                        self.tokens.push(token!(
                            chars.into_iter().collect::<String>(),
                            TokenType::Number
                        ))
                    } else if tkn.is_alphabetic() {
                        let mut chars: Vec<char> = vec![tkn];
                        loop {
                            let chr = src.peek();
                            match chr {
                                Some(chr) => {
                                    if !chr.is_alphabetic() {
                                        break;
                                    }
                                    let t = src.next();
                                    chars.push(t.unwrap());
                                }
                                None => {
                                    break;
                                }
                            }
                        }

                        let val = chars.into_iter().collect::<String>();
                        let mut token_type = TokenType::Identifier;
                        if val.as_str() == NIL_KEYWORD {
                            token_type = TokenType::Nil
                        } else if val.as_str() == LET_KEYWORD {
                            token_type = TokenType::Let
                        }
                        self.tokens.push(token!(val, token_type))
                    } else {
                        panic!("unhandled token: {}", tkn)
                    }
                }
            }
        }

        self.tokens.push(token!("EOF".to_string(), TokenType::EOF));
        Ok(self.tokens.clone())
    }
}
