use std::collections::HashMap;

use rail::{
    ast::{
        ast_printer::TreePrinter,
        parser::Parser,
        types::{CompilationUnit, TypeEnv, Typer},
    },
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
    let printer = TreePrinter::new(&parser.builder.arena);
    printer.print(exp);

    let env = TypeEnv::new();
    let typer = Typer::new(&env);
    let mut unit = CompilationUnit {
        arena: parser.builder.arena,
        types: HashMap::new(),
    };

    let ty = typer.calculate_type(&unit.arena, &mut unit.types, exp);
    dbg!(ty);
}
