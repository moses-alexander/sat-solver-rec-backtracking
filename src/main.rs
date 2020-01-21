mod expr;
mod test;

fn main() {
    println!("...");
    println!("{:?}", expr::solve::reduce(test::test::test_expr1()));
}
