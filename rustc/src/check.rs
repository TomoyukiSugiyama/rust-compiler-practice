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
    // Calculate line number and starting byte index of the line
    let mut line_num = 1;
    let mut line_start = 0;
    for (idx, ch) in exp.char_indices() {
        if idx >= pos {
            break;
        }
        if ch == '\n' {
            line_num += 1;
            line_start = idx + ch.len_utf8();
        }
    }
    // Determine the end of the current line
    let line_end = exp[line_start..]
        .find('\n')
        .map(|i| line_start + i)
        .unwrap_or(exp.len());
    let line = &exp[line_start..line_end];
    // Calculate the column (character offset) within the line
    let col = exp[line_start..pos].chars().count();
    // Print the line with its number
    println!("{} | {}", line_num, line);
    // Build and print the caret line with message
    let line_num_str = line_num.to_string();
    let prefix_spaces = " ".repeat(line_num_str.len()) + " | ";
    println!("{}{}^ {}", prefix_spaces, " ".repeat(col), msg);
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

    #[test]
    #[should_panic(expected = "test error")]
    fn test_error_tok() {
        let head = tokenize("x");
        let tok = head.next.as_ref().unwrap();
        error_tok(&tok, "test error");
    }
}
