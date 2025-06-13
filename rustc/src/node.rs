use crate::check::{error_tok, expect_token};
use crate::token::*;
use crate::variable::Variable;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    // Sequence of two statements: execute first, then second
    Seq {
        first: Box<Node>,
        second: Box<Node>,
    },
    // Literals
    Num {
        value: u64,
    },
    StringSlice {
        value: String,
    },
    // Variables and functions
    Var {
        offset: u64,
    },
    Function {
        name: String,
        args: Vec<Node>,
        body: Box<Node>,
    },
    Call {
        name: String,
        args: Vec<Node>,
    },
    Syscall {
        name: String,
        args: Vec<Node>,
    },
    // Assignment
    Assign {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    // Arithmetic operations
    Add {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Sub {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Mul {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Div {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    // Comparison operations
    Eq {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Ne {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Lt {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Gt {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Le {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Ge {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    // Control flow
    Return {
        expr: Box<Node>,
    },
    If {
        cond: Box<Node>,
        then_stmt: Box<Node>,
        else_stmt: Option<Box<Node>>,
    },
    While {
        cond: Box<Node>,
        body: Box<Node>,
    },
    For {
        init: Box<Node>,
        cond: Box<Node>,
        update: Box<Node>,
        body: Box<Node>,
    },
    // Pointer operations
    Deref {
        expr: Box<Node>,
    },
    Addr {
        expr: Box<Node>,
    },
    ArrayAssign {
        offset: u64,
        elements: Vec<Node>,
    },
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
        root = Node::Seq {
            first: Box::new(root),
            second: Box::new(next),
        };
    }
    root
}

// function ::= 'fn' ident '(' function_args? ')' ('->' type)? '{' stmt* '}'
fn function(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    // consume 'fn'
    expect_next(toks, TokenKind::Fn);
    // parse function name
    let tok = toks.next().unwrap();
    let name = if let TokenKind::Ident { name } = tok.kind.clone() {
        name
    } else {
        error_tok(&tok, "expected identifier");
    };
    // expect '('
    expect_next(toks, TokenKind::LParen);
    // parse optional parameters only if the next token is an identifier
    let mut args_vec = Vec::new();
    if let Some(peek) = toks.peek() {
        if let TokenKind::Ident { name: _ } = peek.kind {
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
        Node::Num { value: 0 }
    } else {
        let mut iter = stmts.into_iter();
        let mut b = iter.next().unwrap();
        for next in iter {
            b = Node::Seq {
                first: Box::new(b),
                second: Box::new(next),
            };
        }
        b
    };
    Node::Function {
        name: name,
        args: args_vec,
        body: Box::new(body),
    }
}

// stmt ::= expr ';' |
//          '{' stmt* '}' |
//          'let' ident '=' expr ';' |
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
                return Node::Return {
                    expr: Box::new(node),
                };
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
                    root = Node::Seq {
                        first: Box::new(root),
                        second: Box::new(next),
                    };
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
                return Node::If {
                    cond: Box::new(cond),
                    then_stmt: Box::new(then_stmt),
                    else_stmt: else_stmt,
                };
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
                return Node::While {
                    cond: Box::new(cond),
                    body: Box::new(body),
                };
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
                return Node::For {
                    init: Box::new(init),
                    cond: Box::new(cond),
                    update: Box::new(update),
                    body: Box::new(body),
                };
            }
            TokenKind::Let => {
                // parse let statement: 'let' ident '=' expr ';'
                toks.next();
                // expect identifier
                let tok_ident = toks.next().unwrap();
                let name = if let TokenKind::Ident { name } = tok_ident.kind.clone() {
                    name
                } else {
                    error_tok(&tok_ident, "expected identifier after 'let'");
                };
                // expect '='
                expect_next(toks, TokenKind::Assign);
                // array literal assignment: let name = [expr, ...];
                if let Some(peek) = toks.peek() {
                    if peek.kind == TokenKind::LBracket {
                        toks.next(); // consume '['
                        let mut elements = Vec::new();
                        // parse elements if not empty
                        if let Some(peek2) = toks.peek() {
                            if peek2.kind != TokenKind::RBracket {
                                elements.push(expr(toks, vars));
                                while let Some(tok2) = toks.peek() {
                                    if tok2.kind == TokenKind::Comma {
                                        toks.next();
                                        elements.push(expr(toks, vars));
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                        expect_next(toks, TokenKind::RBracket);
                        expect_next(toks, TokenKind::Semicolon);
                        // check for duplicate variable
                        if vars.find(&name).is_some() {
                            error_tok(&tok_ident, "variable already declared");
                        }
                        // allocate new variable offset
                        let last = vars.next.as_ref().map(|v| v.offset).unwrap_or(vars.offset);
                        let new_off = last + 8;
                        vars.push(name.clone(), new_off);
                        return Node::ArrayAssign {
                            offset: new_off,
                            elements,
                        };
                    }
                }
                // parse expression
                let rhs = expr(toks, vars);
                // expect ';'
                expect_next(toks, TokenKind::Semicolon);
                // check for duplicate variable
                if vars.find(&name).is_some() {
                    error_tok(&tok_ident, "variable already declared");
                }
                // allocate new variable offset
                let last = vars.next.as_ref().map(|v| v.offset).unwrap_or(vars.offset);
                let new_off = last + 8;
                vars.push(name.clone(), new_off);
                // return assignment node
                return Node::Assign {
                    lhs: Box::new(Node::Var { offset: new_off }),
                    rhs: Box::new(rhs),
                };
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
fn expr(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
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
                lhs = Node::Assign {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
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
                lhs = Node::Eq {
                    lhs: Box::new(lhs),
                    rhs: Box::new(relational(toks, vars)),
                };
            }
            TokenKind::Ne => {
                toks.next();
                lhs = Node::Ne {
                    lhs: Box::new(lhs),
                    rhs: Box::new(relational(toks, vars)),
                };
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
                lhs = Node::Lt {
                    lhs: Box::new(lhs),
                    rhs: Box::new(add(toks, vars)),
                };
            }
            TokenKind::Gt => {
                toks.next();
                lhs = Node::Gt {
                    lhs: Box::new(lhs),
                    rhs: Box::new(add(toks, vars)),
                };
            }
            TokenKind::Le => {
                toks.next();
                lhs = Node::Le {
                    lhs: Box::new(lhs),
                    rhs: Box::new(add(toks, vars)),
                };
            }
            TokenKind::Ge => {
                toks.next();
                lhs = Node::Ge {
                    lhs: Box::new(lhs),
                    rhs: Box::new(add(toks, vars)),
                };
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
                lhs = Node::Add {
                    lhs: Box::new(lhs),
                    rhs: Box::new(mul(toks, vars)),
                };
            }
            TokenKind::Minus => {
                toks.next();
                lhs = Node::Sub {
                    lhs: Box::new(lhs),
                    rhs: Box::new(mul(toks, vars)),
                };
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
                lhs = Node::Mul {
                    lhs: Box::new(lhs),
                    rhs: Box::new(unary(toks, vars)),
                };
            }
            TokenKind::Slash => {
                toks.next();
                lhs = Node::Div {
                    lhs: Box::new(lhs),
                    rhs: Box::new(unary(toks, vars)),
                };
            }
            _ => break,
        }
    }
    lhs
}

// unary ::= ('+' | '-')? primary | ('*' | '&') unary
fn unary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    if let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Plus => {
                toks.next();
                return primary(toks, vars);
            }
            TokenKind::Minus => {
                toks.next();
                return Node::Sub {
                    lhs: Box::new(Node::Num { value: 0 }),
                    rhs: Box::new(primary(toks, vars)),
                };
            }
            TokenKind::Star => {
                toks.next();
                return Node::Deref {
                    expr: Box::new(unary(toks, vars)),
                };
            }
            TokenKind::Amp => {
                toks.next();
                return Node::Addr {
                    expr: Box::new(unary(toks, vars)),
                };
            }
            _ => {}
        }
    }
    primary(toks, vars)
}

// primary ::= number |
//             ident '[' args? ']' |
//             ident ('(' args? ')')? |
//             '(' expr ')' |
//             string |
fn primary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let tok = toks.next().unwrap();
    match tok.kind {
        TokenKind::Number { num } => Node::Num { value: num },
        TokenKind::String { value } => Node::StringSlice { value },
        TokenKind::LParen => {
            // Parse sub-expression
            let node = expr(toks, vars);
            expect_next(toks, TokenKind::RParen);
            node
        }
        TokenKind::Ident { name } => {
            let name = name.clone();
            // array indexing: name[expr]
            if let Some(tok2) = toks.peek() {
                if tok2.kind == TokenKind::LBracket {
                    toks.next(); // consume '['
                    // parse index expression
                    let idx = expr(toks, vars);
                    // expect ']'
                    expect_next(toks, TokenKind::RBracket);
                    // determine variable offset (allocate if not exist)
                    let offset = if let Some(off) = vars.find(&name) {
                        off
                    } else {
                        let last = vars.next.as_ref().map(|v| v.offset).unwrap_or(vars.offset);
                        let new_off = last + 8;
                        vars.push(name.clone(), new_off);
                        new_off
                    };
                    // compute address: &name - idx * 8, then dereference
                    return Node::Deref {
                        expr: Box::new(Node::Sub {
                            lhs: Box::new(Node::Addr {
                                expr: Box::new(Node::Var { offset }),
                            }),
                            rhs: Box::new(Node::Mul {
                                lhs: Box::new(idx),
                                rhs: Box::new(Node::Num { value: 8 }),
                            }),
                        }),
                    };
                }
            }
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
                    // Special handling for write as a system call
                    if name == "write" {
                        return Node::Syscall {
                            name: "write".to_string(),
                            args: args_vec,
                        };
                    }
                    return Node::Call {
                        name,
                        args: args_vec,
                    };
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
            Node::Var { offset }
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
        let name = if let TokenKind::Ident { name } = &tok.kind {
            name.clone()
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
        args.push(Node::Var { offset: off });
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
            Node::Function {
                name: "main".to_string(),
                args: vec![],
                body: Box::new(Node::Num { value: 42 }),
            }
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
            Node::Seq {
                first: Box::new(Node::Function {
                    name: "main".to_string(),
                    args: vec![],
                    body: Box::new(Node::Num { value: 1 }),
                }),
                second: Box::new(Node::Function {
                    name: "foo".to_string(),
                    args: vec![],
                    body: Box::new(Node::Num { value: 2 }),
                }),
            }
        );
    }

    #[test]
    fn test_program_return_in_function() {
        let mut iter = tokenize("fn main() { return 3; }").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Function {
                name: "main".to_string(),
                args: vec![],
                body: Box::new(Node::Return {
                    expr: Box::new(Node::Num { value: 3 }),
                }),
            }
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
            Node::Seq {
                first: Box::new(Node::Function {
                    name: "mainA".to_string(),
                    args: vec![],
                    body: Box::new(Node::Num { value: 1 }),
                }),
                second: Box::new(Node::Function {
                    name: "mainB".to_string(),
                    args: vec![],
                    body: Box::new(Node::Return {
                        expr: Box::new(Node::Num { value: 2 }),
                    }),
                }),
            }
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
            Node::If {
                cond: Box::new(Node::Num { value: 1 }),
                then_stmt: Box::new(Node::Num { value: 2 }),
                else_stmt: None,
            }
        );
    }

    #[test]
    fn test_stmt_if_else() {
        let mut iter = tokenize("if (1) 2; else 3;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = stmt(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::If {
                cond: Box::new(Node::Num { value: 1 }),
                then_stmt: Box::new(Node::Num { value: 2 }),
                else_stmt: Some(Box::new(Node::Num { value: 3 })),
            }
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
            Node::While {
                cond: Box::new(Node::Num { value: 1 }),
                body: Box::new(Node::Num { value: 2 }),
            }
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
            Node::For {
                init: Box::new(Node::Num { value: 1 }),
                cond: Box::new(Node::Num { value: 2 }),
                update: Box::new(Node::Num { value: 3 }),
                body: Box::new(Node::Num { value: 4 }),
            }
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
        assert_eq!(node, Node::Num { value: 42 });
    }

    #[test]
    fn test_expr_add_sub() {
        let mut iter = tokenize("1+2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Add {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
    }

    #[test]
    fn test_expr_precedence() {
        let mut iter = tokenize("1+2*3").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        let expected = Node::Add {
            lhs: Box::new(Node::Num { value: 1 }),
            rhs: Box::new(Node::Mul {
                lhs: Box::new(Node::Num { value: 2 }),
                rhs: Box::new(Node::Num { value: 3 }),
            }),
        };
        assert_eq!(node, expected);
    }

    #[test]
    fn test_expr_parens_mul() {
        let mut iter = tokenize("(1+2)*3").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        let expected = Node::Mul {
            lhs: Box::new(Node::Add {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }),
            rhs: Box::new(Node::Num { value: 3 }),
        };
        assert_eq!(node, expected);
    }

    #[test]
    fn test_primary_parens() {
        let mut iter = tokenize("(42)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = primary(&mut iter, &mut vars);
        assert_eq!(node, Node::Num { value: 42 });
    }

    #[test]
    fn test_expr_nested_parens() {
        let mut iter = tokenize("((1+2))").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Add {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
    }

    #[test]
    fn test_expr_assign() {
        let mut iter = tokenize("1=2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Assign {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
    }

    #[test]
    fn test_expr_eq_ne() {
        let mut it1 = tokenize("1==2").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let n1 = expr(&mut it1, &mut vars);
        assert_eq!(
            n1,
            Node::Eq {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
        let mut it2 = tokenize("1!=2").into_iter().peekable();
        let mut vars2 = Variable::new("".to_string(), 0, None);
        let n2 = expr(&mut it2, &mut vars2);
        assert_eq!(
            n2,
            Node::Ne {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
    }

    #[test]
    fn test_expr_relational() {
        let mut vars = Variable::new("".to_string(), 0, None);
        let mut it_lt = tokenize("1<2").into_iter().peekable();
        assert_eq!(
            expr(&mut it_lt, &mut vars),
            Node::Lt {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
        let mut vars2 = Variable::new("".to_string(), 0, None);
        let mut it_gt = tokenize("2>1").into_iter().peekable();
        assert_eq!(
            expr(&mut it_gt, &mut vars2),
            Node::Gt {
                lhs: Box::new(Node::Num { value: 2 }),
                rhs: Box::new(Node::Num { value: 1 }),
            }
        );
        let mut vars3 = Variable::new("".to_string(), 0, None);
        let mut it_le = tokenize("1<=1").into_iter().peekable();
        assert_eq!(
            expr(&mut it_le, &mut vars3),
            Node::Le {
                lhs: Box::new(Node::Num { value: 1 }),
                rhs: Box::new(Node::Num { value: 1 }),
            }
        );
        let mut vars4 = Variable::new("".to_string(), 0, None);
        let mut it_ge = tokenize("2>=2").into_iter().peekable();
        assert_eq!(
            expr(&mut it_ge, &mut vars4),
            Node::Ge {
                lhs: Box::new(Node::Num { value: 2 }),
                rhs: Box::new(Node::Num { value: 2 }),
            }
        );
    }

    #[test]
    fn test_ident_offset() {
        let mut iter = tokenize("a").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = primary(&mut iter, &mut vars);
        assert_eq!(node, Node::Var { offset: 8 });
    }

    #[test]
    fn test_ident_repeated_offset() {
        let mut iter = tokenize("a a").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let first = primary(&mut iter, &mut vars);
        let second = primary(&mut iter, &mut vars);
        assert_eq!(first, Node::Var { offset: 8 });
        assert_eq!(second, Node::Var { offset: 8 });
    }

    #[test]
    fn test_assign_ident() {
        let mut iter = tokenize("a=1").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Assign {
                lhs: Box::new(Node::Var { offset: 8 }),
                rhs: Box::new(Node::Num { value: 1 }),
            }
        );
    }

    #[test]
    fn test_call_no_args() {
        let mut iter = tokenize("foo()").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        assert_eq!(
            primary(&mut iter, &mut vars),
            Node::Call {
                name: "foo".to_string(),
                args: vec![],
            }
        );
    }

    #[test]
    fn test_call_one_arg() {
        let mut iter = tokenize("foo(42)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        assert_eq!(
            primary(&mut iter, &mut vars),
            Node::Call {
                name: "foo".to_string(),
                args: vec![Node::Num { value: 42 }],
            }
        );
    }

    #[test]
    fn test_call_multiple_args() {
        let mut iter = tokenize("foo(1,2)").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        assert_eq!(
            primary(&mut iter, &mut vars),
            Node::Call {
                name: "foo".to_string(),
                args: vec![Node::Num { value: 1 }, Node::Num { value: 2 }],
            }
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

    #[test]
    fn test_unary_plus() {
        let mut iter = tokenize("+42").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(node, Node::Num { value: 42 });
    }

    #[test]
    fn test_unary_minus() {
        let mut iter = tokenize("-42").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Sub {
                lhs: Box::new(Node::Num { value: 0 }),
                rhs: Box::new(Node::Num { value: 42 }),
            }
        );
    }

    #[test]
    fn test_unary_deref() {
        let mut iter = tokenize("*42").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Deref {
                expr: Box::new(Node::Num { value: 42 }),
            }
        );
    }

    #[test]
    fn test_unary_addr() {
        let mut iter = tokenize("&42").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = expr(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Addr {
                expr: Box::new(Node::Num { value: 42 }),
            }
        );
    }

    #[test]
    fn test_primary_string() {
        let mut iter = tokenize(r#""Hello, world!""#).into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = primary(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::StringSlice {
                value: "Hello, world!".to_string()
            }
        );
    }
}
