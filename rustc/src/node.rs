use crate::token::*;
use crate::variable::Variable;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    // Sequence of two statements: execute first, then second
    Seq(Box<Node>, Box<Node>),
    Num(u64),
    Var(u64),
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
}

// program ::= stmt*
pub fn program(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut stmts = Vec::new();
    while let Some(tok) = toks.peek() {
        if let TokenKind::Eof = tok.kind {
            break;
        }
        stmts.push(stmt(toks, vars));
    }
    // Fold statements into nested Seq nodes
    let mut iter = stmts.into_iter();
    let mut root = iter.next().unwrap();
    for next in iter {
        root = Node::Seq(Box::new(root), Box::new(next));
    }
    root
}

// stmt ::= expr ';' | 'return' expr ';' | 'if' '(' expr ')' stmt ('else' stmt)?
fn stmt(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    // return statement
    if let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Return {
            toks.next();
            let node = expr(toks, vars);
            let tok = toks.next().unwrap();
            if tok.kind != TokenKind::Semicolon {
                panic!("expected ';' but found {:?}", tok.kind);
            }
            return Node::Return(Box::new(node));
        }
        if tok.kind == TokenKind::If {
            // parse if statement: 'if' '(' expr ')' stmt ('else' stmt)?
            toks.next();
            // expect '('
            let tok = toks.next().unwrap();
            if tok.kind != TokenKind::LParen {
                panic!("expected '(' but found {:?}", tok.kind);
            }
            // parse condition
            let cond = expr(toks, vars);
            // expect ')'
            let tok = toks.next().unwrap();
            if tok.kind != TokenKind::RParen {
                panic!("expected ')' but found {:?}", tok.kind);
            }
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
    }
    // expression statement
    let node = expr(toks, vars);
    let tok = toks.next().unwrap();
    if tok.kind != TokenKind::Semicolon {
        panic!("expected ';' but found {:?}", tok.kind);
    }
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
        if tok.kind == TokenKind::Assign {
            toks.next();
            let rhs = assign(toks, vars);
            lhs = Node::Assign(Box::new(lhs), Box::new(rhs));
        }
    }
    lhs
}

// equality ::= relational (( '==' | '!=' ) relational)*
fn equality(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = relational(toks, vars);
    while let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::EqEq {
            toks.next();
            lhs = Node::Eq(Box::new(lhs), Box::new(relational(toks, vars)));
            continue;
        } else if tok.kind == TokenKind::Ne {
            toks.next();
            lhs = Node::Ne(Box::new(lhs), Box::new(relational(toks, vars)));
            continue;
        }
        break;
    }
    lhs
}

// relational ::= add (('<' | '>' | '<=' | '>=') add)*
fn relational(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = add(toks, vars);
    while let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Lt {
            toks.next();
            lhs = Node::Lt(Box::new(lhs), Box::new(add(toks, vars)));
            continue;
        } else if tok.kind == TokenKind::Gt {
            toks.next();
            lhs = Node::Gt(Box::new(lhs), Box::new(add(toks, vars)));
            continue;
        } else if tok.kind == TokenKind::Le {
            toks.next();
            lhs = Node::Le(Box::new(lhs), Box::new(add(toks, vars)));
            continue;
        } else if tok.kind == TokenKind::Ge {
            toks.next();
            lhs = Node::Ge(Box::new(lhs), Box::new(add(toks, vars)));
            continue;
        }
        break;
    }
    lhs
}

// add ::= mul (('+' | '-') mul)*
fn add(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = mul(toks, vars);
    while let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Plus {
            toks.next();
            lhs = Node::Add(Box::new(lhs), Box::new(mul(toks, vars)));
            continue;
        } else if tok.kind == TokenKind::Minus {
            toks.next();
            lhs = Node::Sub(Box::new(lhs), Box::new(mul(toks, vars)));
            continue;
        }
        break;
    }
    lhs
}

// mul ::= unary (('*' | '/') unary)*
fn mul(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = unary(toks, vars);
    while let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Star {
            toks.next();
            lhs = Node::Mul(Box::new(lhs), Box::new(unary(toks, vars)));
            continue;
        } else if tok.kind == TokenKind::Slash {
            toks.next();
            lhs = Node::Div(Box::new(lhs), Box::new(unary(toks, vars)));
            continue;
        }
        break;
    }
    lhs
}

// unary ::= ('+' | '-')? primary
fn unary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    if let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Plus {
            toks.next();
            return primary(toks, vars);
        } else if tok.kind == TokenKind::Minus {
            toks.next();
            return Node::Sub(Box::new(Node::Num(0)), Box::new(primary(toks, vars)));
        }
    }
    primary(toks, vars)
}

// primary ::= number | '(' expr ')' | ident
fn primary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let tok = toks.next().unwrap();
    match tok.kind {
        TokenKind::Number(n) => Node::Num(n),
        TokenKind::LParen => {
            // Parse sub-expression
            let node = expr(toks, vars);
            // Expect closing ')'
            let closing = toks.next().unwrap();
            if closing.kind == TokenKind::RParen {
                node
            } else {
                panic!("expected ')' but found {:?}", closing.kind);
            }
        }
        TokenKind::Ident(ref ident) => {
            let name = ident.clone();
            // 既存変数のオフセットを取得、未定義なら新規作成
            let offset = if let Some(off) = vars.find(&name) {
                off
            } else {
                // 直前に push された変数のオフセット、未登録なら vars.offset（0）
                let last = vars.next.as_ref().map(|v| v.offset).unwrap_or(vars.offset);
                let new_off = last + 8;
                vars.push(name.clone(), new_off);
                new_off
            };
            Node::Var(offset)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // program function tests
    #[test]
    fn test_program_single_stmt() {
        let mut iter = tokenize("42;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(node, Node::Num(42));
    }

    #[test]
    fn test_program_two_stmts() {
        let mut iter = tokenize("1;2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Seq(Box::new(Node::Num(1)), Box::new(Node::Num(2)))
        );
    }

    #[test]
    fn test_program_return() {
        let mut iter = tokenize("return 3;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(node, Node::Return(Box::new(Node::Num(3))));
    }

    #[test]
    fn test_program_seq_return() {
        let mut iter = tokenize("1;return 2;").into_iter().peekable();
        let mut vars = Variable::new("".to_string(), 0, None);
        let node = program(&mut iter, &mut vars);
        assert_eq!(
            node,
            Node::Seq(
                Box::new(Node::Num(1)),
                Box::new(Node::Return(Box::new(Node::Num(2))))
            )
        );
    }
}
