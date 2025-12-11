use rail::codegen::CodeGen;
use rail::lexer::Lexer;
use rail::parser::Parser;
use rail::printer::TreePrinter;
use rail::semantic::TypeEnv;
use rail::typechecker::Typer;
use rail::vm::Vm;

fn main() {
    println!("hello from rail");
    let args: Vec<String> = std::env::args().into_iter().collect();
    let file = std::path::PathBuf::from(&args[1]);
    let source = std::fs::read_to_string(file).unwrap();

    let lexer = Lexer::new(&source);
    let parser = Parser::new(lexer);
    let syntax = parser.parse();

    let printer = TreePrinter::new(&syntax);
    printer.print();

    let env = TypeEnv::new();
    let typer = Typer::new(&env);
    let module = typer.check(syntax).unwrap();

    let mut compiler = CodeGen::new();
    let program = compiler.compile(module);

    let mut vm = Vm::from(&program);
    let _ = vm.run();
}
