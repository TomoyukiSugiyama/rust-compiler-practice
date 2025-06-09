#[derive(Debug, PartialEq, Eq)]
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
    LParen,
    RParen,
    Eof,
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

/// Report an error at the given position in `exp` and exit.
fn error_at(exp: &str, pos: usize, msg: &str) -> ! {
    // Print the input and a caret under the error position
    println!("{}", exp);
    println!("{}^ {}", " ".repeat(pos), msg);
    panic!("{}", msg);
}

pub fn expect_number(cur: &Token, exp: &str) -> u64 {
    match &cur.kind {
        TokenKind::Number(n) => *n,
        _ => error_at(exp, cur.pos, "数ではありません"),
    }
}

pub fn expect_operator(cur: &Token, exp: &str) -> TokenKind {
    match &cur.kind {
        TokenKind::Plus => TokenKind::Plus,
        TokenKind::Minus => TokenKind::Minus,
        TokenKind::Star => TokenKind::Star,
        TokenKind::Slash => TokenKind::Slash,
        TokenKind::EqEq => TokenKind::EqEq,
        TokenKind::Ne => TokenKind::Ne,
        TokenKind::Lt => TokenKind::Lt,
        TokenKind::Le => TokenKind::Le,
        TokenKind::Gt => TokenKind::Gt,
        TokenKind::Ge => TokenKind::Ge,
        TokenKind::Assign => TokenKind::Assign,
        TokenKind::Semicolon => TokenKind::Semicolon,
        TokenKind::LParen => TokenKind::LParen,
        TokenKind::RParen => TokenKind::RParen,
        _ => error_at(exp, cur.pos, "演算子ではありません"),
    }
}

pub fn at_eof(cur: &Token) -> bool {
    matches!(&cur.kind, TokenKind::Eof)
}

/// Tokenizes an arithmetic expression into a linked list of tokens.
/// Supports positive integers, identifiers, operators, and delimiters.
/// Returns the head `Token`, whose chained `next` pointers end with an `Eof` token.
pub fn tokenize(exp: &str) -> Token {
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
            // Read full number starting at i
            let start = i;
            let mut num = 0u64;
            while let Some(&(_, dch)) = chars.peek() {
                if dch.is_ascii_digit() {
                    num = num
                        .checked_mul(10)
                        .and_then(|n| n.checked_add(dch.to_digit(10).unwrap() as u64))
                        .unwrap();
                    chars.next();
                } else {
                    break;
                }
            }
            tail = tail.push(TokenKind::Number(num), start);
        } else if c.is_ascii_alphabetic() {
            let start = i;
            let mut ident = String::new();
            while let Some(&(_, dch)) = chars.peek() {
                if dch.is_ascii_alphanumeric() {
                    ident.push(dch);
                    chars.next();
                } else {
                    break;
                }
            }
            tail = tail.push(TokenKind::Ident(ident), start);
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
                '(' => {
                    chars.next();
                    TokenKind::LParen
                }
                ')' => {
                    chars.next();
                    TokenKind::RParen
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
    fn test_expect_number_and_operator() {
        let head = tokenize("1+2");
        let num_tok = head.next.as_ref().unwrap();
        assert_eq!(expect_number(num_tok, "1+2"), 1);
        let op_tok = num_tok.next.as_ref().unwrap();
        assert_eq!(expect_operator(op_tok, "1+2"), TokenKind::Plus);
    }

    #[test]
    fn test_at_eof() {
        let head = tokenize("1");
        let first = head.next.as_ref().unwrap();
        let eof_tok = first.next.as_ref().unwrap();
        assert!(at_eof(eof_tok));
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
    #[should_panic(expected = "数ではありません")]
    fn test_expect_number_panic() {
        let head = tokenize("1+2");
        let op_tok = head.next.as_ref().unwrap().next.as_ref().unwrap();
        expect_number(op_tok, "1+2");
    }

    #[test]
    #[should_panic(expected = "演算子ではありません")]
    fn test_expect_operator_panic() {
        let head = tokenize("123");
        let num_tok = head.next.as_ref().unwrap();
        expect_operator(num_tok, "123");
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
    fn test_iter_eof_flag() {
        let mut iter = tokenize("1").into_iter();
        let one = iter.next().unwrap();
        assert!(!at_eof(&one));
        let eof = iter.next().unwrap();
        assert!(at_eof(&eof));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_expect_operator_paren() {
        let head = tokenize("(");
        let paren = head.next.as_ref().unwrap();
        assert_eq!(expect_operator(paren, "("), TokenKind::LParen);
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
        let input = "+ - * / == != < <= > >= = ; ( )";
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
                TokenKind::Eof,
            ]
        );
    }
}
