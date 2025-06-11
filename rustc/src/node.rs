use crate::check::{error_tok, expect_token};
use crate::token::*;
use crate::variable::Variable;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    // Sequence of two statements: execute first, then second
    Seq(Box<Node>, Box<Node>),
    Num(u64),
    Var(u64),
    Function(String, Vec<Node>, Box<Node>),
    // Function call with optional arguments: name(arg1, arg2, ...)
    Call(String, Vec<Node>),
    Assign(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Ne(Box<Node>, Box<Node>),
    Lt(Box<Node>, Box<Node>),
    Gt(Box<Node>, Box<Node>),
    Le(Box<Node>, Box<Node>),
    Ge(Box<Node>, Box<Node>),
    Return(Box<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
    While(Box<Node>, Box<Node>),
    For(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
}

fn expect_next(toks: &mut Peekable<TokenIter>, expected: TokenKind) -> Token {
    let tok = toks.next().unwrap();
    expect_token(&tok, &expected);
    tok
}

// program ::= function*
pub fn program(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut funcs = Vec::new();
    while let Some(tok) = toks.peek() {
        if let TokenKind::Eof = tok.kind {
            break;
        }
        // parse a function definition
        funcs.push(function(toks, vars));
    }
    // Fold functions into nested Seq nodes
    let mut iter = funcs.into_iter();
    let mut root = iter.next().unwrap();
    for next in iter {
        root = Node::Seq(Box::new(root), Box::new(next));
    }
    root
}

// function ::= 'fn' ident '(' function_args? ')' ('->' type)? '{' stmt* '}'
fn function(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    // consume 'fn'
    expect_next(toks, TokenKind::Fn);
    // parse function name
    let tok = toks.next().unwrap();
    let name = if let TokenKind::Ident(ident) = tok.kind.clone() {
        ident
    } else {
        error_tok(&tok, "expected identifier");
    };
    // expect '('
    expect_next(toks, TokenKind::LParen);
    // parse optional parameters only if the next token is an identifier
    let mut args_vec = Vec::new();
    if let Some(peek) = toks.peek() {
        if let TokenKind::Ident(_) = peek.kind {
            args_vec = function_args(toks, vars);
        }
    }
    // expect ')'
    expect_next(toks, TokenKind::RParen);
    // optional return type '-> type'
    if let Some(peek) = toks.peek() {
        if peek.kind == TokenKind::Arrow {
            toks.next();
            // parse type (only i32 supported)
            expect_next(toks, TokenKind::I32);
        }
    }
    // expect '{'
    expect_next(toks, TokenKind::LBrace);
    // parse body statements
    let mut stmts = Vec::new();
    while let Some(peek) = toks.peek() {
        if peek.kind == TokenKind::RBrace {
            break;
        }
        // error if EOF reached before closing brace
        if peek.kind == TokenKind::Eof {
            error_tok(peek, "expected RBrace");
        }
        stmts.push(stmt(toks, vars));
    }
    // expect '}'
    expect_next(toks, TokenKind::RBrace);
    // fold into a single Node, default to 0 if empty
    let body = if stmts.is_empty() {
        Node::Num(0)
    } else {
        let mut iter = stmts.into_iter();
        let mut b = iter.next().unwrap();
        for next in iter {
            b = Node::Seq(Box::new(b), Box::new(next));
        }
        b
    };
    Node::Function(name, args_vec, Box::new(body))
}

// stmt ::= expr ';' |
//          '{' stmt* '}' |
//          'return' expr ';' |
//          'if' '(' expr ')' stmt ('else' stmt)? |
//          'while' '(' expr ')' stmt |
//          'for' '(' expr ';' expr ';' expr ')' stmt
fn stmt(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    // detect EOF as missing statement
    if let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Eof {
            error_tok(tok, "expected statement");
        }
    }
    if let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Return => {
                toks.next();
                let node = expr(toks, vars);
                expect_next(toks, TokenKind::Semicolon);
                return Node::Return(Box::new(node));
            }
            TokenKind::LBrace => {
                toks.next();
                let mut stmts = Vec::new();
                while let Some(tok) = toks.peek() {
                    if tok.kind == TokenKind::RBrace {
                        break;
                    }
                    // error if EOF before closing brace
                    if tok.kind == TokenKind::Eof {
                        error_tok(tok, "expected RBrace");
                    }
                    stmts.push(stmt(toks, vars));
                }
                expect_next(toks, TokenKind::RBrace);
                let mut iter = stmts.into_iter();
                let mut root = iter.next().unwrap();
                for next in iter {
                    root = Node::Seq(Box::new(root), Box::new(next));
                }
                return root;
            }
            TokenKind::If => {
                // parse if statement: 'if' '(' expr ')' stmt ('else' stmt)?
                toks.next();
                // expect '('
                expect_next(toks, TokenKind::LParen);
                // error if no condition expression
                if let Some(peek) = toks.peek() {
                    if peek.kind == TokenKind::RParen || peek.kind == TokenKind::Eof {
                        error_tok(peek, "expected expression");
                    }
                }
                // parse condition
                let cond = expr(toks, vars);
                // expect ')'
                expect_next(toks, TokenKind::RParen);
                // parse then branch
                let then_stmt = stmt(toks, vars);
                // parse optional else branch
                let else_stmt = if let Some(tok) = toks.peek() {
                    if tok.kind == TokenKind::Else {
                        toks.next();
                        Some(Box::new(stmt(toks, vars)))
                    } else {
                        None
                    }
                } else {
                    None
                };
                return Node::If(Box::new(cond), Box::new(then_stmt), else_stmt);
            }
            TokenKind::While => {
                // parse while statement: 'while' '(' expr ')' stmt
                toks.next();
                // expect '('
                expect_next(toks, TokenKind::LParen);
                // error if no condition expression
                if let Some(peek) = toks.peek() {
                    if peek.kind == TokenKind::RParen || peek.kind == TokenKind::Eof {
                        error_tok(peek, "expected expression");
                    }
                }
                // parse condition
                let cond = expr(toks, vars);
                // expect ')'
                expect_next(toks, TokenKind::RParen);
                // parse body
                let body = stmt(toks, vars);
                return Node::While(Box::new(cond), Box::new(body));
            }
            TokenKind::For => {
                // parse for statement: 'for' '(' expr ';' expr ';' expr ')' stmt
                toks.next();
                // expect '('
                expect_next(toks, TokenKind::LParen);
                // error if missing init expression
                if let Some(peek) = toks.peek() {
                    if peek.kind == TokenKind::Semicolon || peek.kind == TokenKind::Eof {
                        error_tok(peek, "expected expression");
                    }
                }
                // parse init
                let init = expr(toks, vars);
                expect_next(toks, TokenKind::Semicolon);
                // error if missing condition expression
                if let Some(peek) = toks.peek() {
                    if peek.kind == TokenKind::Semicolon || peek.kind == TokenKind::Eof {
                        error_tok(peek, "expected expression");
                    }
                }
                // parse condition
                let cond = expr(toks, vars);
                expect_next(toks, TokenKind::Semicolon);
                // error if missing update expression
                if let Some(peek) = toks.peek() {
                    if peek.kind == TokenKind::RParen || peek.kind == TokenKind::Eof {
                        error_tok(peek, "expected expression");
                    }
                }
                // parse update
                let update = expr(toks, vars);
                // expect ')'
                expect_next(toks, TokenKind::RParen);
                // parse body
                let body = stmt(toks, vars);
                return Node::For(
                    Box::new(init),
                    Box::new(cond),
                    Box::new(update),
                    Box::new(body),
                );
            }
            _ => {}
        }
    }
    // expression statement
    let node = expr(toks, vars);
    expect_next(toks, TokenKind::Semicolon);
    node
}

// expr ::= assign
pub fn expr(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    assign(toks, vars)
}

// assign ::= equality ('=' assign)?
fn assign(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = equality(toks, vars);
    if let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Assign => {
                toks.next();
                let rhs = assign(toks, vars);
                lhs = Node::Assign(Box::new(lhs), Box::new(rhs));
            }
            _ => {}
        }
    }
    lhs
}

// equality ::= relational (( '==' | '!=' ) relational)*
fn equality(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = relational(toks, vars);
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::EqEq => {
                toks.next();
                lhs = Node::Eq(Box::new(lhs), Box::new(relational(toks, vars)));
            }
            TokenKind::Ne => {
                toks.next();
                lhs = Node::Ne(Box::new(lhs), Box::new(relational(toks, vars)));
            }
            _ => break,
        }
    }
    lhs
}

// relational ::= add (('<' | '>' | '<=' | '>=') add)*
fn relational(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = add(toks, vars);
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Lt => {
                toks.next();
                lhs = Node::Lt(Box::new(lhs), Box::new(add(toks, vars)));
            }
            TokenKind::Gt => {
                toks.next();
                lhs = Node::Gt(Box::new(lhs), Box::new(add(toks, vars)));
            }
            TokenKind::Le => {
                toks.next();
                lhs = Node::Le(Box::new(lhs), Box::new(add(toks, vars)));
            }
            TokenKind::Ge => {
                toks.next();
                lhs = Node::Ge(Box::new(lhs), Box::new(add(toks, vars)));
            }
            _ => break,
        }
    }
    lhs
}

// add ::= mul (('+' | '-') mul)*
fn add(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = mul(toks, vars);
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Plus => {
                toks.next();
                lhs = Node::Add(Box::new(lhs), Box::new(mul(toks, vars)));
            }
            TokenKind::Minus => {
                toks.next();
                lhs = Node::Sub(Box::new(lhs), Box::new(mul(toks, vars)));
            }
            _ => break,
        }
    }
    lhs
}

// mul ::= unary (('*' | '/') unary)*
fn mul(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = unary(toks, vars);
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Star => {
                toks.next();
                lhs = Node::Mul(Box::new(lhs), Box::new(unary(toks, vars)));
            }
            TokenKind::Slash => {
                toks.next();
                lhs = Node::Div(Box::new(lhs), Box::new(unary(toks, vars)));
            }
            _ => break,
        }
    }
    lhs
}

// unary ::= ('+' | '-')? primary
fn unary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    if let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Plus => {
                toks.next();
                return primary(toks, vars);
            }
            TokenKind::Minus => {
                toks.next();
                return Node::Sub(Box::new(Node::Num(0)), Box::new(primary(toks, vars)));
            }
            _ => {}
        }
    }
    primary(toks, vars)
}

// primary ::= number |
//             ident ('(' args? ')')? |
//             '(' expr ')' |
fn primary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let tok = toks.next().unwrap();
    match tok.kind {
        TokenKind::Number(n) => Node::Num(n),
        TokenKind::LParen => {
            // Parse sub-expression
            let node = expr(toks, vars);
            expect_next(toks, TokenKind::RParen);
            node
        }
        TokenKind::Ident(ref ident) => {
            let name = ident.clone();
            // function call: name(args?)
            if let Some(tok2) = toks.peek() {
                if tok2.kind == TokenKind::LParen {
                    toks.next(); // consume '('
                    // parse zero or more args
                    let args_vec = if let Some(peek) = toks.peek() {
                        if peek.kind == TokenKind::RParen {
                            Vec::new()
                        } else {
                            args(toks, vars)
                        }
                    } else {
                        Vec::new()
                    };
                    // expect closing ')'
                    expect_next(toks, TokenKind::RParen);
                    return Node::Call(name, args_vec);
                }
            }
            // variable
            let offset = if let Some(off) = vars.find(&name) {
                off
            } else {
                let last = vars.next.as_ref().map(|v| v.offset).unwrap_or(vars.offset);
                let new_off = last + 8;
                vars.push(name.clone(), new_off);
                new_off
            };
            Node::Var(offset)
        }
        _ => error_tok(&tok, &format!("unexpected token: {:?}", tok.kind)),
    }
}

// function_args ::= ident ':' type (',' ident ':' type)*
fn function_args(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Vec<Node> {
    let mut args = Vec::new();
    // Parse one or more function_arg ::= ident ':' type, separated by commas
    loop {
        // identifier
        let tok = toks.next().unwrap();
        let name = if let TokenKind::Ident(ident) = &tok.kind {
            ident.clone()
        } else {
            error_tok(&tok, "expected identifier");
        };
        // ':'
        expect_next(toks, TokenKind::Colon);
        // type (e.g., 'i32')
        expect_next(toks, TokenKind::I32);
        // assign new offset for this parameter
        let off = if let Some(off) = vars.find(&name) {
            off
        } else {
            let last = vars.next.as_ref().map(|v| v.offset).unwrap_or(vars.offset);
            let new_off = last + 8;
            vars.push(name.clone(), new_off);
            new_off
        };
        // represent parameter as a Var node
        args.push(Node::Var(off));
        // if a comma follows, consume it and continue parsing
        if let Some(peek) = toks.peek() {
            if peek.kind == TokenKind::Comma {
                toks.next();
                continue;
            }
        }
        break;
    }
    args
}

// args ::= expr (',' expr)*
fn args(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Vec<Node> {
    let mut args = Vec::new();
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::RParen => break,
            _ => {
                args.push(expr(toks, vars));
                if let Some(tok2) = toks.peek() {
                    match tok2.kind {
                        TokenKind::Comma => {
                            toks.next();
                            continue;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    //=== Function parsing (happy path) ===
    #[test]
    fn test_program_single_function() {
        let mut iter = tokenize("fn main() { 42; }").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Function("main".to_string(), vec![], Box::new(Node::Num(42)))
        );
    }

    #[test]
    fn test_program_two_functions() {
        let mut iter = tokenize("fn main() { 1; } fn foo() { 2; }")
            .into_iter()
            .peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Seq(
                Box::new(Node::Function(
                    "main".to_string(),
                    vec![],
                    Box::new(Node::Num(1))
                )),
                Box::new(Node::Function(
                    "foo".to_string(),
                    vec![],
                    Box::new(Node::Num(2))
                ))
            )
        );
    }

    #[test]
    fn test_program_return_in_function() {
        let mut iter = tokenize("fn main() { return 3; }").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Function(
                "main".to_string(),
                vec![],
                Box::new(Node::Return(Box::new(Node::Num(3))))
            )
        );
    }

    #[test]
    fn test_program_two_functions_with_return() {
        let mut iter = tokenize("fn mainA() { 1; } fn mainB() { return 2; }")
            .into_iter()
            .peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Seq(
                Box::new(Node::Function(
                    "mainA".to_string(),
                    vec![],
                    Box::new(Node::Num(1))
                )),
                Box::new(Node::Function(
                    "mainB".to_string(),
                    vec![],
                    Box::new(Node::Return(Box::new(Node::Num(2))))
                ))
            )
        );
    }

    //=== Function parsing error tests ===
    #[test]
    #[should_panic(expected = "expected identifier")]
    fn test_error_fn_missing_ident() {
        let mut iter = tokenize("fn() {}").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected LParen")]
    fn test_error_fn_missing_lparen() {
        let mut iter = tokenize("fn main) { }").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected RParen")]
    fn test_error_fn_missing_rparen() {
        let mut iter = tokenize("fn main( {}").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected LBrace")]
    fn test_error_fn_missing_lbrace() {
        let mut iter = tokenize("fn main() )").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected RBrace")]
    fn test_error_fn_missing_rbrace() {
        let mut iter = tokenize("fn main() { 1;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected Colon")]
    fn test_error_fn_args_missing_colon() {
        let mut iter = tokenize("fn foo(a i32) {}").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected I32")]
    fn test_error_fn_args_missing_type() {
        let mut iter = tokenize("fn foo(a:)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        program(&mut iter, &mut vars);
    }

    //=== If parsing (happy path) ===
    #[test]
    fn test_stmt_if() {
        let mut iter = tokenize("if (1) 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = stmt(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::If(Box::new(Node::Num(1)), Box::new(Node::Num(2)), None)
        );
    }

    #[test]
    fn test_stmt_if_else() {
        let mut iter = tokenize("if (1) 2; else 3;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = stmt(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::If(
                Box::new(Node::Num(1)),
                Box::new(Node::Num(2)),
                Some(Box::new(Node::Num(3)))
            )
        );
    }

    //=== If parsing error tests ===
    #[test]
    #[should_panic(expected = "expected LParen")]
    fn test_error_if_missing_lparen() {
        let mut iter = tokenize("if 1) 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected RParen")]
    fn test_error_if_missing_rparen() {
        let mut iter = tokenize("if (1 2 3;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected expression")]
    fn test_error_if_missing_condition() {
        let mut iter = tokenize("if () 1;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected statement")]
    fn test_error_if_missing_then_branch() {
        let mut iter = tokenize("if (1)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    //=== While parsing (happy path) ===
    #[test]
    fn test_stmt_while() {
        let mut iter = tokenize("while (1) 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = stmt(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::While(Box::new(Node::Num(1)), Box::new(Node::Num(2)))
        );
    }

    //=== While parsing error tests ===
    #[test]
    #[should_panic(expected = "expected LParen")]
    fn test_error_while_missing_lparen() {
        let mut iter = tokenize("while 1) 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected RParen")]
    fn test_error_while_missing_rparen() {
        let mut iter = tokenize("while (1 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected expression")]
    fn test_error_while_missing_condition() {
        let mut iter = tokenize("while () 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected statement")]
    fn test_error_while_missing_body() {
        let mut iter = tokenize("while (1)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    //=== For parsing (happy path) ===
    #[test]
    fn test_stmt_for() {
        let mut iter = tokenize("for (1;2;3) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = stmt(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::For(
                Box::new(Node::Num(1)),
                Box::new(Node::Num(2)),
                Box::new(Node::Num(3)),
                Box::new(Node::Num(4)),
            )
        );
    }

    //=== For parsing error tests ===
    #[test]
    #[should_panic(expected = "expected LParen")]
    fn test_error_for_missing_lparen() {
        let mut iter = tokenize("for 1;2;3) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected Semicolon")]
    fn test_error_for_missing_semicolon1() {
        let mut iter = tokenize("for (1 2;3) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected Semicolon")]
    fn test_error_for_missing_semicolon2() {
        let mut iter = tokenize("for (1;2 3) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected expression")]
    fn test_error_for_missing_init() {
        let mut iter = tokenize("for (;2;3) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected expression")]
    fn test_error_for_missing_cond() {
        let mut iter = tokenize("for (1;;3) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected expression")]
    fn test_error_for_missing_update() {
        let mut iter = tokenize("for (1;2;) 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected RParen")]
    fn test_error_for_missing_rparen() {
        let mut iter = tokenize("for (1;2;3 4;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected statement")]
    fn test_error_for_missing_body() {
        let mut iter = tokenize("for (1;2;3)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }

    //=== Miscellaneous parsing tests ===
    #[test]
    fn test_primary() {
        let mut iter = tokenize("42").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = primary(&mut iter, &mut vars);
        assert_eq!(node, Node::Num(42));
    }

    #[test]
    fn test_expr_add_sub() {
        let mut iter = tokenize("1+2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Add(Box::new(Node::Num(1)), Box::new(Node::Num(2)))
        );
    }

    #[test]
    fn test_expr_precedence() {
        let mut iter = tokenize("1+2*3").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        let expected = Node::Add(
            Box::new(Node::Num(1)),
            Box::new(Node::Mul(Box::new(Node::Num(2)), Box::new(Node::Num(3)))),
        );
        assert_eq!(node, expected);
    }

    #[test]
    fn test_expr_parens_mul() {
        let mut iter = tokenize("(1+2)*3").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        let expected = Node::Mul(
            Box::new(Node::Add(Box::new(Node::Num(1)), Box::new(Node::Num(2)))),
            Box::new(Node::Num(3)),
        );
        assert_eq!(node, expected);
    }

    #[test]
    fn test_primary_parens() {
        let mut iter = tokenize("(42)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = primary(&mut iter, &mut vars);
        assert_eq!(node, Node::Num(42));
    }

    #[test]
    fn test_expr_nested_parens() {
        let mut iter = tokenize("((1+2))").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Add(Box::new(Node::Num(1)), Box::new(Node::Num(2)))
        );
    }

    #[test]
    fn test_expr_assign() {
        let mut iter = tokenize("1=2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Assign(Box::new(Node::Num(1)), Box::new(Node::Num(2)))
        );
    }

    #[test]
    fn test_expr_eq_ne() {
        let mut it1 = tokenize("1==2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let n1 = expr(&mut it1, &mut vars);
        assert_eq!(n1, Node::Eq(Box::new(Node::Num(1)), Box::new(Node::Num(2))));
        let mut it2 = tokenize("1!=2").into_iter().peekable();
        let mut vars2 = Variable::new("".to_string(), 0, None);
        let n2 = expr(&mut it2, &mut vars2);
        assert_eq!(n2, Node::Ne(Box::new(Node::Num(1)), Box::new(Node::Num(2))));
    }

    #[test]
    fn test_expr_relational() {
        let mut vars = Variable::new("".to_string(), 0, None);
        let mut it_lt = tokenize("1<2").into_iter().peekable();
        assert_eq!(
            expr(&mut it_lt, &mut vars),
            Node::Lt(Box::new(Node::Num(1)), Box::new(Node::Num(2)))
        );
        let mut vars2 = Variable::new("".to_string(), 0, None);
        let mut it_gt = tokenize("2>1").into_iter().peekable();
        assert_eq!(
            expr(&mut it_gt, &mut vars2),
            Node::Gt(Box::new(Node::Num(2)), Box::new(Node::Num(1)))
        );
        let mut vars3 = Variable::new("".to_string(), 0, None);
        let mut it_le = tokenize("1<=1").into_iter().peekable();
        assert_eq!(
            expr(&mut it_le, &mut vars3),
            Node::Le(Box::new(Node::Num(1)), Box::new(Node::Num(1)))
        );
        let mut vars4 = Variable::new("".to_string(), 0, None);
        let mut it_ge = tokenize("2>=2").into_iter().peekable();
        assert_eq!(
            expr(&mut it_ge, &mut vars4),
            Node::Ge(Box::new(Node::Num(2)), Box::new(Node::Num(2)))
        );
    }

    #[test]
    fn test_ident_offset() {
        let mut iter = tokenize("a").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = primary(&mut iter, &mut vars);
        assert_eq!(node, Node::Var(8));
    }

    #[test]
    fn test_ident_repeated_offset() {
        let mut iter = tokenize("a a").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let first = primary(&mut iter, &mut vars);
        let second = primary(&mut iter, &mut vars);
        assert_eq!(first, Node::Var(8));
        assert_eq!(second, Node::Var(8));
    }

    #[test]
    fn test_assign_ident() {
        let mut iter = tokenize("a=1").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Assign(Box::new(Node::Var(8)), Box::new(Node::Num(1)))
        );
    }

    #[test]
    fn test_call_no_args() {
        let mut iter = tokenize("foo()").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        assert_eq!(
            primary(&mut iter, &mut vars),
            Node::Call("foo".to_string(), vec![])
        );
    }

    #[test]
    fn test_call_one_arg() {
        let mut iter = tokenize("foo(42)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        assert_eq!(
            primary(&mut iter, &mut vars),
            Node::Call("foo".to_string(), vec![Node::Num(42)])
        );
    }

    #[test]
    fn test_call_multiple_args() {
        let mut iter = tokenize("foo(1,2)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        assert_eq!(
            primary(&mut iter, &mut vars),
            Node::Call("foo".to_string(), vec![Node::Num(1), Node::Num(2)])
        );
    }

    //=== Miscellaneous error tests ===
    #[test]
    #[should_panic(expected = "expected RParen")]
    fn test_error_primary_missing_rparen() {
        let mut iter = tokenize("(1+2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        primary(&mut iter, &mut vars);
    }

    #[test]
    #[should_panic(expected = "expected Semicolon")]
    fn test_error_stmt_missing_semicolon() {
        let mut iter = tokenize("42").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        stmt(&mut iter, &mut vars);
    }
}
