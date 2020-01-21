// data type and function defns

#[derive(Debug)]
pub enum Expr {
    Var(char),
    Const(bool),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    NULL
}

pub mod solve {

    use super::Expr;

    fn unconst (e: Expr) -> Option<bool> {
        match e {
            Expr::Const(b) => Some(b),
            _              => None
        }
    }

    fn choice (e1: Option<Expr>, e2: Option<Expr>)  -> Option<Expr> {
        match e1 {
            Some(e) => Some(e),
            None    => match e2 {
                            Some(e) => Some(e),
                            None    => None
            }
        }
    }

    fn eval_not(e: Expr) -> Expr {
        match reduce(e) {
            Expr::Const(false) => Expr::Const(true),
            Expr::Const(true)  => Expr::Const(false),
            _                  => Expr::NULL
        }
    }

    fn eval_and (e1: Expr, e2: Expr) -> Expr {
        let a = reduce(e1);
        let b = reduce(e2);
        match a {
            Expr::Const(false) => Expr::Const(false),
            Expr::Const(true) => match b {
                                    Expr::Const(true)  => Expr::Const(true),
                                    Expr::Const(false) => Expr::Const(false),
                                    Expr::Var(_c)      => Expr::And(
                                                            Box::new(a),
                                                            Box::new(b)),
                                    _                  => Expr::NULL
            }
            Expr::Var(_c) => Expr::And(Box::new(a), Box::new(b)),
            _             => Expr::NULL
        }
    }

    // can't use variables when pattern matching so dup. code
    fn eval_or (e1: Expr, e2: Expr) -> Expr {
        let a = reduce(e1);
        let b = reduce(e2);
        match a {
            Expr::Const(true) => Expr::Const(true),
            Expr::Const(false) => match b {
                                    Expr::Const(false)  => Expr::Const(false),
                                    Expr::Const(true) => Expr::Const(true),
                                    Expr::Var(_c)      => Expr::And(
                                                            Box::new(a),
                                                            Box::new(b)),
                                    _                  => Expr::NULL
            }
            Expr::Var(_c) => Expr::And(Box::new(a), Box::new(b)),
            _             => Expr::NULL
        }
    }

    fn find (e: Expr) -> Option<Expr> {
        match e {
            Expr::Var(c)         => Some(Expr::Var(c)),
            Expr::Const(_)       => None,
            Expr::Not(e1)        => find(*e1),
            Expr::And(e1, e2)    => choice(find(*e1), find(*e2)),
            Expr::Or(e1, e2)     => choice(find(*e1), find(*e2)),
            Expr::NULL           => None
        }
    }

    fn guess (var: char, constant: bool, e: Expr) -> Expr {
        match e {
            Expr::Var(c)      => if c == var {return Expr::Const(constant);}
                                 else        {return Expr::Var(c);},
            Expr::Const(b)    => Expr::Const(b),
            Expr::Not(e1)     => guess(var, constant, *e1),
            Expr::And(e1, e2) => Expr::And(
                                    Box::new(guess(var, constant, *e1)),
                                    Box::new(guess(var, constant, *e2))),
            Expr::Or(e1, e2) => Expr::Or(
                                    Box::new(guess(var, constant, *e1)),
                                    Box::new(guess(var, constant, *e2))),
            Expr::NULL       => Expr::NULL
        }
    }

    pub fn reduce (e: Expr) -> Expr {
        match e {
            Expr::Var(c)      => Expr::Var(c),
            Expr::Const(b)    => Expr::Const(b),
            Expr::Not(e)      => eval_not(*e),
            Expr::And(e1, e2) => eval_and(*e1, *e2),
            Expr::Or(e1, e2)  => eval_or(*e1, *e2),
            _                 => Expr::NULL
        }
    }

    fn satisfy (e: Expr) -> bool {
        match find(e) {
            None                 => unconst(e).unwrap(),
            Some(Expr::Var(var)) =>
                satisfy(reduce(guess(var, true, e))) ||
                satisfy(reduce(guess(var, false, e))),
                // this case cannot ever happen, jus here to make compiler happy
            _                    => false

        }
    
    }
}