#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Start,
    Number(u64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Assign,
    Semicolon,
    Comma,
    Colon,
    LParen,
    RParen,
    Eof,
    Return,
    If,
    Else,
    While,
    For,
    Fn,
    LBrace,
    RBrace,
    I32,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize, // byte index in the input string
    pub next: Option<Box<Token>>,
}

impl Token {
    /// Append a new token of the given kind and position after this one and return a mutable reference to it.
    pub fn push(&mut self, kind: TokenKind, pos: usize) -> &mut Token {
        self.next = Some(Box::new(Token {
            kind,
            pos,
            next: None,
        }));
        self.next.as_mut().unwrap()
    }
}

use crate::check::{error_at, set_current_exp};

use std::iter::Peekable;
use std::str::CharIndices;

/// Reads a sequence of digits and returns the parsed number.
fn read_number(chars: &mut Peekable<CharIndices>) -> u64 {
    let mut num = 0u64;
    while let Some(&(_, ch)) = chars.peek() {
        if ch.is_ascii_digit() {
            num = num
                .checked_mul(10)
                .and_then(|n| n.checked_add(ch.to_digit(10).unwrap() as u64))
                .unwrap();
            chars.next();
        } else {
            break;
        }
    }
    num
}

/// Reads an alphanumeric sequence and returns it as a string.
fn read_ident(chars: &mut Peekable<CharIndices>) -> String {
    let mut word = String::new();
    while let Some(&(_, ch)) = chars.peek() {
        if ch.is_ascii_alphanumeric() {
            word.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    word
}

/// Tokenizes an arithmetic expression into a linked list of tokens.
/// Supports positive integers, identifiers, operators, and delimiters.
/// Returns the head `Token`, whose chained `next` pointers end with an `Eof` token.
pub fn tokenize(exp: &str) -> Token {
    set_current_exp(exp);
    // Build linked list with a sentinel head (pos=0)
    let mut head = Token {
        kind: TokenKind::Start,
        pos: 0,
        next: None,
    };
    let mut tail = &mut head;
    // Iterate with char_indices to track positions
    let mut chars = exp.char_indices().peekable();
    while let Some(&(i, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() {
            let start = i;
            let num = read_number(&mut chars);
            tail = tail.push(TokenKind::Number(num), start);
        } else if c.is_ascii_alphabetic() {
            let start = i;
            let word = read_ident(&mut chars);
            let kind = match word.as_str() {
                "return" => TokenKind::Return,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "while" => TokenKind::While,
                "for" => TokenKind::For,
                "fn" => TokenKind::Fn,
                "i32" => TokenKind::I32,
                _ => TokenKind::Ident(word),
            };
            tail = tail.push(kind, start);
        } else {
            // Operators and delimiters
            let pos = i;
            let kind = match c {
                '=' => {
                    chars.next();
                    if let Some(&(_, '=')) = chars.peek() {
                        chars.next();
                        TokenKind::EqEq
                    } else {
                        TokenKind::Assign
                    }
                }
                '!' => {
                    chars.next();
                    if let Some(&(_, '=')) = chars.peek() {
                        chars.next();
                        TokenKind::Ne
                    } else {
                        error_at(exp, i, "無効な文字です");
                    }
                }
                '<' => {
                    chars.next();
                    if let Some(&(_, '=')) = chars.peek() {
                        chars.next();
                        TokenKind::Le
                    } else {
                        TokenKind::Lt
                    }
                }
                '>' => {
                    chars.next();
                    if let Some(&(_, '=')) = chars.peek() {
                        chars.next();
                        TokenKind::Ge
                    } else {
                        TokenKind::Gt
                    }
                }
                '+' => {
                    chars.next();
                    TokenKind::Plus
                }
                '-' => {
                    chars.next();
                    TokenKind::Minus
                }
                '*' => {
                    chars.next();
                    TokenKind::Star
                }
                '/' => {
                    chars.next();
                    TokenKind::Slash
                }
                ';' => {
                    chars.next();
                    TokenKind::Semicolon
                }
                ',' => {
                    chars.next();
                    TokenKind::Comma
                }
                ':' => {
                    chars.next();
                    TokenKind::Colon
                }
                '(' => {
                    chars.next();
                    TokenKind::LParen
                }
                ')' => {
                    chars.next();
                    TokenKind::RParen
                }
                '{' => {
                    chars.next();
                    TokenKind::LBrace
                }
                '}' => {
                    chars.next();
                    TokenKind::RBrace
                }
                _ => {
                    error_at(exp, i, "無効な文字です");
                }
            };
            tail = tail.push(kind, pos);
        }
    }
    // Append EOF token at end of input
    tail.push(TokenKind::Eof, exp.len());
    head
}

/// An iterator over tokens (skips the initial Start sentinel)
pub struct TokenIter {
    current: Option<Token>,
}

impl Iterator for TokenIter {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut tok) = self.current.take() {
            self.current = tok.next.take().map(|b| *b);
            Some(tok)
        } else {
            None
        }
    }
}

impl IntoIterator for Token {
    type Item = Token;
    type IntoIter = TokenIter;
    fn into_iter(self) -> Self::IntoIter {
        TokenIter {
            current: self.next.map(|b| *b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let head = tokenize("12 + 34 -5");
        let mut t = &head;
        let mut kinds: Vec<&TokenKind> = Vec::new();
        while let Some(next) = &t.next {
            t = next;
            kinds.push(&t.kind);
        }
        assert_eq!(
            kinds,
            vec![
                &TokenKind::Number(12),
                &TokenKind::Plus,
                &TokenKind::Number(34),
                &TokenKind::Minus,
                &TokenKind::Number(5),
                &TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_into_iterator() {
        let kinds: Vec<TokenKind> = tokenize("12 + 34 -5")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Number(12),
                TokenKind::Plus,
                TokenKind::Number(34),
                TokenKind::Minus,
                TokenKind::Number(5),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    #[should_panic(expected = "無効な文字です")]
    fn test_tokenize_panic_on_invalid_char() {
        let _ = tokenize("?");
    }

    #[test]
    fn test_tokenize_parens() {
        let kinds: Vec<TokenKind> = tokenize("(1+2)*3")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::LParen,
                TokenKind::Number(1),
                TokenKind::Plus,
                TokenKind::Number(2),
                TokenKind::RParen,
                TokenKind::Star,
                TokenKind::Number(3),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_ident() {
        let kinds: Vec<TokenKind> = tokenize("foo bar")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident("foo".to_string()),
                TokenKind::Ident("bar".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_all_operators() {
        let input = "+ - * / == != < <= > >= = ; ( ) { } return if else while for fn";
        let kinds: Vec<TokenKind> = tokenize(input).into_iter().map(|tok| tok.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::EqEq,
                TokenKind::Ne,
                TokenKind::Lt,
                TokenKind::Le,
                TokenKind::Gt,
                TokenKind::Ge,
                TokenKind::Assign,
                TokenKind::Semicolon,
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::LBrace,
                TokenKind::RBrace,
                TokenKind::Return,
                TokenKind::If,
                TokenKind::Else,
                TokenKind::While,
                TokenKind::For,
                TokenKind::Fn,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_comma() {
        let kinds: Vec<TokenKind> = tokenize(",").into_iter().map(|tok| tok.kind).collect();
        assert_eq!(kinds, vec![TokenKind::Comma, TokenKind::Eof]);
    }

    #[test]
    fn test_tokenize_call_tokens() {
        let kinds: Vec<TokenKind> = tokenize("foo(1,2)")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident("foo".to_string()),
                TokenKind::LParen,
                TokenKind::Number(1),
                TokenKind::Comma,
                TokenKind::Number(2),
                TokenKind::RParen,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_fn_keyword() {
        let kinds: Vec<TokenKind> = tokenize("fn").into_iter().map(|tok| tok.kind).collect();
        assert_eq!(kinds, vec![TokenKind::Fn, TokenKind::Eof,]);
    }

    #[test]
    fn test_tokenize_fn_declaration() {
        let kinds: Vec<TokenKind> = tokenize("fn foo() { return 42; }")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Fn,
                TokenKind::Ident("foo".to_string()),
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::LBrace,
                TokenKind::Return,
                TokenKind::Number(42),
                TokenKind::Semicolon,
                TokenKind::RBrace,
                TokenKind::Eof,
            ]
        );
    }
}
