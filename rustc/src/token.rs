#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Start,
    Number { num: u64 },
    Ident(String),
    String(String),
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
    Arrow,
    Amp,
    Let,
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

/// Table of keyword strings to token kinds.
const KEYWORDS: &[(&str, TokenKind)] = &[
    ("return", TokenKind::Return),
    ("if", TokenKind::If),
    ("else", TokenKind::Else),
    ("while", TokenKind::While),
    ("for", TokenKind::For),
    ("fn", TokenKind::Fn),
    ("let", TokenKind::Let),
    ("i32", TokenKind::I32),
];

/// Looks up a word in the keyword table and returns the corresponding TokenKind.
fn lookup_keyword(word: &str) -> Option<TokenKind> {
    for &(kw, ref kind) in KEYWORDS {
        if kw == word {
            return Some(kind.clone());
        }
    }
    None
}

/// Slice of operator lexemes mapped to their TokenKind, sorted by descending length to match longest first.
const OPERATORS: &[(&str, TokenKind)] = &[
    ("==", TokenKind::EqEq),
    ("!=", TokenKind::Ne),
    ("<=", TokenKind::Le),
    (">=", TokenKind::Ge),
    ("->", TokenKind::Arrow),
    ("<", TokenKind::Lt),
    (">", TokenKind::Gt),
    ("=", TokenKind::Assign),
    ("+", TokenKind::Plus),
    ("-", TokenKind::Minus),
    ("*", TokenKind::Star),
    ("/", TokenKind::Slash),
    (";", TokenKind::Semicolon),
    (",", TokenKind::Comma),
    (":", TokenKind::Colon),
    ("(", TokenKind::LParen),
    (")", TokenKind::RParen),
    ("{", TokenKind::LBrace),
    ("}", TokenKind::RBrace),
    ("&", TokenKind::Amp),
];

/// Reads an operator or delimiter and returns the TokenKind by matching against `OPERATORS`.
fn read_operator(chars: &mut Peekable<CharIndices>, exp: &str, pos: usize) -> TokenKind {
    let rest = &exp[pos..];
    for &(s, ref kind) in OPERATORS {
        if rest.starts_with(s) {
            // consume the characters of s
            for _ in 0..s.chars().count() {
                chars.next();
            }
            return kind.clone();
        }
    }
    // no operator matched; report error
    error_at(exp, pos, "無効な文字です");
}

/// Skips characters until the end of the current line (including newline), assuming the next two chars are "//".
fn skip_line_comment(chars: &mut Peekable<CharIndices>) {
    // consume "//"
    chars.next();
    chars.next();
    while let Some((_, ch)) = chars.next() {
        if ch == '\n' {
            break;
        }
    }
}

/// Skips a C-style comment block, reporting an error to stdout if not closed.
fn skip_block_comment(chars: &mut Peekable<CharIndices>, _exp: &str, start_pos: usize) {
    // consume "/*"
    chars.next();
    chars.next();
    let mut found_end = false;
    while let Some((_, ch)) = chars.next() {
        if ch == '*' {
            if let Some(&(_, next_ch)) = chars.peek() {
                if next_ch == '/' {
                    chars.next();
                    found_end = true;
                    break;
                }
            }
        }
    }
    if !found_end {
        println!(
            "コメントの閉じタグ */ が見つかりませんでした at pos {}",
            start_pos
        );
    }
}

/// Reads a string literal and returns it as a string.
fn read_string(chars: &mut Peekable<CharIndices>, exp: &str, start_pos: usize) -> String {
    let mut s = String::new();
    // Skip opening quote
    chars.next();
    while let Some((_, ch)) = chars.next() {
        if ch == '"' {
            return s;
        }
        s.push(ch);
    }
    // If we get here, we hit EOF before finding closing quote
    error_at(exp, start_pos, "文字列が閉じられていません");
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
        let rest = &exp[i..];
        if c.is_whitespace() {
            chars.next();
            continue;
        } else if rest.starts_with("//") {
            // skip single-line comment
            skip_line_comment(&mut chars);
            continue;
        } else if rest.starts_with("/*") {
            // skip multi-line comment
            skip_block_comment(&mut chars, exp, i);
            continue;
        } else if c == '"' {
            // Handle string literal
            let start = i;
            let s = read_string(&mut chars, exp, start);
            tail = tail.push(TokenKind::String(s), start);
            continue;
        } else if c.is_ascii_digit() {
            let start = i;
            let num = read_number(&mut chars);
            tail = tail.push(TokenKind::Number { num }, start);
            continue;
        } else if c.is_ascii_alphabetic() {
            let start = i;
            let word = read_ident(&mut chars);
            let kind = lookup_keyword(&word).unwrap_or(TokenKind::Ident(word));
            tail = tail.push(kind, start);
            continue;
        } else {
            // Operators and delimiters
            let pos = i;
            let kind = read_operator(&mut chars, exp, pos);
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
    fn test_tokenize_empty_or_whitespace_only() {
        let kinds: Vec<TokenKind> = tokenize("   ").into_iter().map(|tok| tok.kind).collect();
        assert_eq!(kinds, vec![TokenKind::Eof]);
    }

    #[test]
    #[should_panic(expected = "無効な文字です")]
    fn test_tokenize_panic_on_invalid_char() {
        let _ = tokenize("?");
    }

    #[test]
    fn test_tokenize_numbers_and_arithmetic() {
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
                &TokenKind::Number { num: 12 },
                &TokenKind::Plus,
                &TokenKind::Number { num: 34 },
                &TokenKind::Minus,
                &TokenKind::Number { num: 5 },
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
                TokenKind::Number { num: 12 },
                TokenKind::Plus,
                TokenKind::Number { num: 34 },
                TokenKind::Minus,
                TokenKind::Number { num: 5 },
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
    fn test_tokenize_alphanumeric_ident() {
        let kinds: Vec<TokenKind> = tokenize("foo123 bar456")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident("foo123".into()),
                TokenKind::Ident("bar456".into()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_i32_keyword_and_ident_mix() {
        let kinds: Vec<TokenKind> = tokenize("i32 i32foo fooi32")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::I32,
                TokenKind::Ident("i32foo".into()),
                TokenKind::Ident("fooi32".into()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_fn_keyword() {
        let kinds: Vec<TokenKind> = tokenize("fn").into_iter().map(|tok| tok.kind).collect();
        assert_eq!(kinds, vec![TokenKind::Fn, TokenKind::Eof]);
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
                TokenKind::Number { num: 42 },
                TokenKind::Semicolon,
                TokenKind::RBrace,
                TokenKind::Eof,
            ]
        );
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
                TokenKind::Number { num: 1 },
                TokenKind::Plus,
                TokenKind::Number { num: 2 },
                TokenKind::RParen,
                TokenKind::Star,
                TokenKind::Number { num: 3 },
                TokenKind::Eof,
            ]
        );
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
                TokenKind::Number { num: 1 },
                TokenKind::Comma,
                TokenKind::Number { num: 2 },
                TokenKind::RParen,
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
    fn test_tokenize_colon() {
        let kinds: Vec<TokenKind> = tokenize(":").into_iter().map(|tok| tok.kind).collect();
        assert_eq!(kinds, vec![TokenKind::Colon, TokenKind::Eof]);
    }

    #[test]
    fn test_tokenize_arrow() {
        let kinds: Vec<TokenKind> = tokenize("->").into_iter().map(|tok| tok.kind).collect();
        assert_eq!(kinds, vec![TokenKind::Arrow, TokenKind::Eof]);
    }

    #[test]
    fn test_tokenize_arrow_with_ident() {
        let kinds: Vec<TokenKind> = tokenize("foo->bar")
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident("foo".to_string()),
                TokenKind::Arrow,
                TokenKind::Ident("bar".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_all_operators() {
        let input = "+ - * / == != < <= > >= = ; ( ) { } return if else while for fn -> & let";
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
                TokenKind::Arrow,
                TokenKind::Amp,
                TokenKind::Let,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_string() {
        let kinds: Vec<TokenKind> = tokenize(r#""Hello, world!""#)
            .into_iter()
            .map(|tok| tok.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::String("Hello, world!".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    #[should_panic(expected = "文字列が閉じられていません")]
    fn test_tokenize_unclosed_string() {
        let _ = tokenize(r#""Hello, world!"#);
    }
}
