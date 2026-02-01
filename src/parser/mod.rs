pub mod lexer;

pub fn parse() {
    let tokens = lexer::tokenize("");
    println!("{:?}", tokens);
    panic!("not implemeted");
}
