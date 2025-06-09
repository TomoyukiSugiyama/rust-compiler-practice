use crate::token::*;
use crate::variable::Variable;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
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
    stmts.into_iter().next().unwrap()
}

// stmt ::= expr ';'
fn stmt(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let node = expr(toks, vars);
    let tok = toks.next().unwrap();
    if let TokenKind::Operator(ref op) = tok.kind {
        if op == ";" {
            return node;
        }
    } else {
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
        if let TokenKind::Operator(ref op) = tok.kind {
            if op == "=" {
                toks.next();
                let rhs = assign(toks, vars);
                lhs = Node::Assign(Box::new(lhs), Box::new(rhs));
            }
        }
    }
    lhs
}

// equality ::= relational (( '==' | '!=' ) relational)*
fn equality(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = relational(toks, vars);
    while let Some(tok) = toks.peek() {
        if let TokenKind::Operator(ref op) = tok.kind {
            if op == "==" {
                toks.next();
                lhs = Node::Eq(Box::new(lhs), Box::new(relational(toks, vars)));
                continue;
            } else if op == "!=" {
                toks.next();
                lhs = Node::Ne(Box::new(lhs), Box::new(relational(toks, vars)));
                continue;
            }
        }
        break;
    }
    lhs
}

// relational ::= add (('<' | '>' | '<=' | '>=') add)*
fn relational(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = add(toks, vars);
    while let Some(tok) = toks.peek() {
        if let TokenKind::Operator(ref op) = tok.kind {
            if op == "<" {
                toks.next();
                lhs = Node::Lt(Box::new(lhs), Box::new(add(toks, vars)));
                continue;
            } else if op == ">" {
                toks.next();
                lhs = Node::Gt(Box::new(lhs), Box::new(add(toks, vars)));
                continue;
            } else if op == "<=" {
                toks.next();
                lhs = Node::Le(Box::new(lhs), Box::new(add(toks, vars)));
                continue;
            } else if op == ">=" {
                toks.next();
                lhs = Node::Ge(Box::new(lhs), Box::new(add(toks, vars)));
                continue;
            }
        }
        break;
    }
    lhs
}

// add ::= mul (('+' | '-') mul)*
fn add(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = mul(toks, vars);
    while let Some(tok) = toks.peek() {
        if let TokenKind::Operator(ref op) = tok.kind {
            if op == "+" {
                toks.next();
                lhs = Node::Add(Box::new(lhs), Box::new(mul(toks, vars)));
                continue;
            } else if op == "-" {
                toks.next();
                lhs = Node::Sub(Box::new(lhs), Box::new(mul(toks, vars)));
                continue;
            }
        }
        break;
    }
    lhs
}

// mul ::= unary (('*' | '/') unary)*
fn mul(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let mut lhs = unary(toks, vars);
    while let Some(tok) = toks.peek() {
        if let TokenKind::Operator(ref op) = tok.kind {
            if op == "*" {
                toks.next();
                lhs = Node::Mul(Box::new(lhs), Box::new(unary(toks, vars)));
                continue;
            } else if op == "/" {
                toks.next();
                lhs = Node::Div(Box::new(lhs), Box::new(unary(toks, vars)));
                continue;
            }
        }
        break;
    }
    lhs
}

// unary ::= ('+' | '-')? primary
fn unary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    if let Some(tok) = toks.peek() {
        if let TokenKind::Operator(ref op) = tok.kind {
            if op == "+" {
                toks.next();
                return primary(toks, vars);
            } else if op == "-" {
                toks.next();
                return Node::Sub(Box::new(Node::Num(0)), Box::new(primary(toks, vars)));
            }
        }
    }
    primary(toks, vars)
}

// primary ::= number | '(' expr ')' | ident
fn primary(toks: &mut Peekable<TokenIter>, vars: &mut Variable) -> Node {
    let tok = toks.next().unwrap();
    match tok.kind {
        TokenKind::Number(n) => Node::Num(n),
        TokenKind::Operator(ref op) if op == "(" => {
            // Parse sub-expression
            let node = expr(toks, vars);
            // Expect closing ')'
            let closing = toks.next().unwrap();
            if let TokenKind::Operator(ref op2) = closing.kind {
                if op2 == ")" {
                    node
                } else {
                    panic!("expected ')' but found {:?}", closing.kind);
                }
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
                // リスト先頭のnextが最後に追加された変数
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
}
