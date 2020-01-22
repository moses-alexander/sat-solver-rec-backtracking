// main

mod expr;
mod test;

fn main() {
    println!("...");
    println!("{:?}", expr::solve::satisfy(test::test::test_expr1()));

    println!("{:?}", expr::solve::satisfy(test::test::test_expr2()));
    println!("{:?}", expr::solve::satisfy(test::test::test_expr3()));

}
