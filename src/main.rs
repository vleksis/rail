use std::collections::HashMap;

use rail::{
    ast::{
        ast_printer::TreePrinter,
        compiler::CodeGen,
        node::ExpressionId,
        parser::Parser,
        types::{CompilationUnit, Type, TypeEnv, Typer},
    },
    bytecode::{OpCode, chunk::Chunk},
    lexer::lexer::Lexer,
    runtime::{function::Function, program::Program},
    vm::vm::VM,
};

fn main() {
    println!("hello from rail");
    let args: Vec<String> = std::env::args().into_iter().collect();
    let file = std::path::PathBuf::from(&args[1]);
    let file = std::fs::read_to_string(file).unwrap();

    let lexer = Lexer::new(&file);
    let mut parser = Parser::new(lexer);
    let exp = parser.parse_expression();

    let arena = parser.builder.arena;
    let printer = TreePrinter::new(&arena);
    printer.print(exp);

    let env = TypeEnv::new();
    let typer = Typer::new(&env);
    let mut types: HashMap<ExpressionId, Type> = HashMap::new();

    let ty = typer.calculate_type(&arena, &mut types, exp);
    dbg!(&types);

    let mut compiler = CodeGen::new();

    let mut chunk = Chunk::new();
    compiler.compile_expr(&arena, &types, &mut chunk, exp);
    chunk.add_instruction(OpCode::Return, 100);

    let main_fn = Function {
        name: "main".to_string(),
        chunk,
        arity: 0,
    };

    let mut program = Program::new();
    program.functions.push(main_fn);

    let mut vm = VM::from(&program);
    let _ = vm.run();
}
