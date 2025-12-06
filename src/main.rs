use ptree::print_tree;
use rail::{
    ast::parser::Parser,
    lexer::{lexer::Lexer, token::TokenKind},
};

fn main() {
    println!("hello from rail");
    let args: Vec<String> = std::env::args().into_iter().collect();
    let file = std::path::PathBuf::from(&args[1]);
    let file = std::fs::read_to_string(file).unwrap();
    let lexer = Lexer::new(&file);
    let mut parser = Parser::new(lexer);
    let exp = parser.parse_expression();
    print_tree(&exp.as_ref()).unwrap();
}
