// tests

pub mod test {

    use super::super::expr::Expr;

    fn tt () -> Expr { return Expr::Const(true); }

    fn ff () -> Expr { return Expr::Const(false); }

    fn var(c: char) -> Expr { return Expr::Var(c); }

    fn not (x: Expr) -> Expr {
        return Expr::Not(Box::new(x));
    }

    fn and (x: Expr, y: Expr) -> Expr {
        return Expr::And(
            Box::new(x),
            Box::new(y));
    }

    fn or (x: Expr, y: Expr) -> Expr {
        return Expr::Or(
            Box::new(x),
            Box::new(y));
    }

    pub fn test_expr1 () -> Expr {
        return not(and(tt(), ff()));
    }

    pub fn test_expr2 () -> Expr {
        return and(var('x'), not(var('x')));
    }

    pub fn test_expr3 () -> Expr {
        return or(var('x'), not(var('x')));
    }

}