// tests

pub mod test {

    use super::super::expr::Expr;

    fn tt () -> Expr { return Expr::Const(true); }

    fn ff () -> Expr { return Expr::Const(false); }

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
        return not(or(and(and(tt(), ff()), tt()), or(tt(), ff())));
    }
}