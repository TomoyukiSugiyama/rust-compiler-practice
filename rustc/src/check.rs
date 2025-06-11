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

/// Panic when an unexpected token is encountered in parsing
pub fn expect_token(cur: &Token, expected_kind: &TokenKind) {
    let exp = CURRENT_EXP.with(|c| c.borrow().clone());
    if cur.kind == *expected_kind {
        return;
    }
    error_at(&exp, cur.pos, &format!("expected {:?}", expected_kind));
}

/// Report a parsing error at the given token with a custom message and print location.
pub fn error_tok(cur: &Token, msg: &str) -> ! {
    let exp = CURRENT_EXP.with(|c| c.borrow().clone());
    error_at(&exp, cur.pos, msg);
}

#[cfg(test)]
mod tests {
    use super::*;

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
