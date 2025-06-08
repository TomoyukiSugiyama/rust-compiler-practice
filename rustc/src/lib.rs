#[derive(Debug)]
pub enum TokenKind {
    Number(u64),
    Operator(char),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    next: Option<Box<Token>>,
}

pub fn new_token(kind: TokenKind, cur: Token) -> Token {
    Token {
        kind,
        next: Some(Box::new(cur)),
    }
}

pub fn consume(cur: Token) -> Option<Token> {
    // Unbox the next token if present
    cur.next.map(|boxed| *boxed)
}

pub fn expect_number(cur: &Token) -> u64 {
    match &cur.kind {
        TokenKind::Number(n) => *n,
        _ => panic!("Expected number, got {:?}", cur.kind),
    }
}

pub fn expect_operator(cur: &Token) -> char {
    match &cur.kind {
        TokenKind::Operator(c) => *c,
        _ => panic!("Expected operator, got {:?}", cur.kind),
    }
}

pub fn at_eof(cur: &Token) -> bool {
    matches!(&cur.kind, TokenKind::Eof)
}

/// Tokenizes an arithmetic expression into a linked list of tokens.
/// Supports positive integers and the '+' and '-' operators.
/// Returns the head `Token`, whose chained `next` pointers end with an `Eof` token.
pub fn tokenize(exp: &str) -> Token {
    // Build linked list with a sentinel head and mutable tail reference
    let mut head = Token {
        kind: TokenKind::Eof,
        next: None,
    };
    let mut tail = &mut head;
    let mut curr_num: Option<u64> = None;

    for c in exp.chars() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_ascii_digit() {
            let d = c.to_digit(10).unwrap() as u64;
            curr_num = Some(curr_num.unwrap_or(0).wrapping_mul(10).wrapping_add(d));
        } else if c == '+' || c == '-' {
            if let Some(n) = curr_num {
                tail.next = Some(Box::new(Token {
                    kind: TokenKind::Number(n),
                    next: None,
                }));
                tail = tail.next.as_mut().unwrap();
                curr_num = None;
            }
            tail.next = Some(Box::new(Token {
                kind: TokenKind::Operator(c),
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();
        } else {
            panic!("Invalid character in expression: {}", c);
        }
    }
    if let Some(n) = curr_num {
        tail.next = Some(Box::new(Token {
            kind: TokenKind::Number(n),
            next: None,
        }));
        tail = tail.next.as_mut().unwrap();
    }
    // Append EOF token
    tail.next = Some(Box::new(Token {
        kind: TokenKind::Eof,
        next: None,
    }));
    head
}
