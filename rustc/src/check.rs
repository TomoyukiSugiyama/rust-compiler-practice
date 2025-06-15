use crate::token::*;
use std::cell::RefCell;
use std::fmt;

/// Represents a parsing error with message and position in input.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub msg: String,
    pub pos: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {}: {}", self.pos, self.msg)
    }
}

impl ParseError {
    /// Unwraps the ParseError by panicking with its message.
    pub fn unwrap(self) -> ! {
        panic!("{}", self.msg);
    }
}

thread_local! {
    static CURRENT_EXP: RefCell<String> = RefCell::new(String::new());
}

/// Set the current input expression for error reporting.
pub fn set_current_exp(exp: &str) {
    CURRENT_EXP.with(|c| *c.borrow_mut() = exp.to_string());
}

/// Reports an error at a specific position in the input and returns a ParseError.
pub fn error_at(exp: &str, pos: usize, msg: &str) -> ParseError {
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
    ParseError {
        msg: msg.to_string(),
        pos,
    }
}

/// Checks whether the current token matches expected kind; returns Ok or ParseError
pub fn expect_token(cur: &Token, expected_kind: &TokenKind) -> Result<(), ParseError> {
    let exp_str = CURRENT_EXP.with(|c| c.borrow().clone());
    if cur.kind == *expected_kind {
        Ok(())
    } else {
        Err(error_at(
            &exp_str,
            cur.pos,
            &format!("expected {:?}", expected_kind),
        ))
    }
}

/// Reports a parsing error at the given token and returns a ParseError.
pub fn error_tok(cur: &Token, msg: &str) -> ParseError {
    let exp_str = CURRENT_EXP.with(|c| c.borrow().clone());
    error_at(&exp_str, cur.pos, msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_token_ok() {
        let head = tokenize(";").unwrap();
        let tok = head.next.as_ref().unwrap();
        expect_token(&tok, &TokenKind::Semicolon).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected Semicolon")]
    fn test_expect_token_panic() {
        let head = tokenize("x").unwrap();
        let tok = head.next.as_ref().unwrap().next.as_ref().unwrap();
        expect_token(&tok, &TokenKind::Semicolon).unwrap();
    }

    #[test]
    #[should_panic(expected = "test error")]
    fn test_error_tok() {
        let head = tokenize("x").unwrap();
        let tok = head.next.as_ref().unwrap();
        error_tok(&tok, "test error").unwrap();
    }

    #[test]
    #[should_panic(expected = "multiline start")]
    fn test_error_at_multiline_line1() {
        let exp = "first line\nsecond line\nthird line";
        // position in first line (pos 0)
        error_at(exp, 0, "multiline start").unwrap();
    }

    #[test]
    #[should_panic(expected = "multiline mid")]
    fn test_error_at_multiline_line2() {
        let exp = "first line\nsecond line foo\nthird line";
        // position of 'foo' in second line
        let pos = exp.find("foo").unwrap();
        error_at(exp, pos, "multiline mid").unwrap();
    }
}
