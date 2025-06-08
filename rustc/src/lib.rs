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

impl Token {
    /// Append a new token of the given kind after this one and return a mutable reference to it.
    pub fn push(&mut self, kind: TokenKind) -> &mut Token {
        self.next = Some(Box::new(Token { kind, next: None }));
        self.next.as_mut().unwrap()
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
    // Build linked list with a sentinel head
    let mut head = Token {
        kind: TokenKind::Eof,
        next: None,
    };
    let mut tail = &mut head;
    // Iterate with peekable to group digits into one number
    let mut chars = exp.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() {
            let mut num = 0u64;
            while let Some(&dch) = chars.peek() {
                if !dch.is_ascii_digit() {
                    break;
                }
                num = num
                    .wrapping_mul(10)
                    .wrapping_add(dch.to_digit(10).unwrap() as u64);
                chars.next();
            }
            tail = tail.push(TokenKind::Number(num));
        } else if c == '+' || c == '-' {
            chars.next();
            tail = tail.push(TokenKind::Operator(c));
        } else {
            panic!("Invalid character in expression: {}", c);
        }
    }
    // Append EOF token
    tail.push(TokenKind::Eof);
    head
}
