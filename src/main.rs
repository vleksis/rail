use rail::{
    ast::{ast_printer::TreePrinter, parser::Parser},
    lexer::lexer::Lexer,
};

fn main() {
    println!("hello from rail");
    let args: Vec<String> = std::env::args().into_iter().collect();
    let file = std::path::PathBuf::from(&args[1]);
    let file = std::fs::read_to_string(file).unwrap();

    let lexer = Lexer::new(&file);
    let mut parser = Parser::new(lexer);
    let exp = parser.parse_expression();
    dbg!(&exp);
    let printer = TreePrinter::new(&parser.builder.arena);
    printer.print(exp);
}
