use XINSh::lexer::lexer;
fn main() {
    println!("{:?}", lexer::lex(r#"echo "hello""#.to_string()));
}

