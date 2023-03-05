use crate::token;
use crate::token::{str_to_keyword, Token};

#[derive(Debug, Clone)]
pub struct SyntaxError;

pub fn tokenize(src: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = vec![];
    let mut src = src.chars().peekable();

    while let Some(tkn) = src.next() {
        match tkn {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '+' | '*' | '/' | '%' => tokens.push(Token::BinOp),
            '=' => tokens.push(Token::Eq),
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
                tokens.push(Token::String {
                    value: chars.into_iter().collect::<String>(),
                })
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
                    tokens.push(Token::Number {
                        value: chars.into_iter().collect::<String>(),
                    })
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
                    if let Some(token) = str_to_keyword(val.as_str()) {
                        tokens.push(token)
                    } else {
                        tokens.push(Token::Identifier { value: val })
                    }
                } else {
                    panic!("unhandled token: {}", tkn)
                }
            }
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens.clone())
}
