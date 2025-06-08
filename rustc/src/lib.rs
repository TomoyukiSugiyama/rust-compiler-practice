#[derive(Debug)]
pub enum TokenKind {
    Number(u64),
    Operator(char),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    pos: usize, // byte index in the input string
    next: Option<Box<Token>>,
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

pub fn consume(cur: Token) -> Option<Token> {
    // Unbox the next token if present
    cur.next.map(|boxed| *boxed)
}

/// Report an error at the given position in `exp` and exit.
fn error_at(exp: &str, pos: usize, msg: &str) -> ! {
    // Print the input and a caret under the error position
    println!("{}", exp);
    println!("{}^ {}", " ".repeat(pos), msg);
    std::process::exit(1);
}

pub fn expect_number(cur: &Token, exp: &str) -> u64 {
    match &cur.kind {
        TokenKind::Number(n) => *n,
        _ => error_at(exp, cur.pos, "数ではありません"),
    }
}

pub fn expect_operator(cur: &Token, exp: &str) -> char {
    match &cur.kind {
        TokenKind::Operator(c) => *c,
        _ => error_at(exp, cur.pos, "演算子ではありません"),
    }
}

pub fn at_eof(cur: &Token) -> bool {
    matches!(&cur.kind, TokenKind::Eof)
}

/// Tokenizes an arithmetic expression into a linked list of tokens.
/// Supports positive integers and the '+' and '-' operators.
/// Returns the head `Token`, whose chained `next` pointers end with an `Eof` token.
pub fn tokenize(exp: &str) -> Token {
    // Build linked list with a sentinel head (pos=0)
    let mut head = Token {
        kind: TokenKind::Eof,
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
            // Read full number starting at i
            let start = i;
            let mut num = 0u64;
            while let Some(&(_, dch)) = chars.peek() {
                if dch.is_ascii_digit() {
                    num = num
                        .wrapping_mul(10)
                        .wrapping_add(dch.to_digit(10).unwrap() as u64);
                    chars.next();
                } else {
                    break;
                }
            }
            tail = tail.push(TokenKind::Number(num), start);
        } else if c == '+' || c == '-' {
            let pos = i;
            chars.next();
            tail = tail.push(TokenKind::Operator(c), pos);
        } else {
            error_at(exp, i, "無効な文字です");
        }
    }
    // Append EOF token at end of input
    tail.push(TokenKind::Eof, exp.len());
    head
}
