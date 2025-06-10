use crate::token::*;
use std::cell::RefCell;

thread_local! {
    static CURRENT_EXP: RefCell<String> = RefCell::new(String::new());
}

/// Set the current input expression for error reporting.
pub fn set_current_exp(exp: &str) {
    CURRENT_EXP.with(|c| *c.borrow_mut() = exp.to_string());
}

pub fn error_at(exp: &str, pos: usize, msg: &str) -> ! {
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

/// Panic when an unexpected token is encountered in parsing
pub fn expect_token(cur: &Token, expected_kind: &TokenKind) {
    let exp = CURRENT_EXP.with(|c| c.borrow().clone());
    if cur.kind == *expected_kind {
        return;
    }
    error_at(&exp, cur.pos, &format!("expected {:?}", expected_kind));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "数ではありません")]
    fn test_expect_number_panic() {
        let head = tokenize("1+2");
        let op_tok = head.next.as_ref().unwrap().next.as_ref().unwrap();
        expect_number(op_tok, "1+2");
    }

    #[test]
    fn test_expect_token_ok() {
        let head = tokenize(";");
        let tok = head.next.as_ref().unwrap();
        expect_token(&tok, &TokenKind::Semicolon);
    }

    #[test]
    #[should_panic(expected = "expected Semicolon")]
    fn test_expect_token_panic() {
        let head = tokenize("x");
        let tok = head.next.as_ref().unwrap().next.as_ref().unwrap();
        expect_token(&tok, &TokenKind::Semicolon);
    }
}
