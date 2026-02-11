use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Subcommand;

use rail::codegen::CodeGen;
use rail::grammar::Syntax;
use rail::lexer::Lexer;
use rail::module::Module;
use rail::parser::Parser;
use rail::runtime::Program;
use rail::semantic::TypeEnv;
use rail::typechecker::Typer;
use rail::vm::Vm;

#[derive(clap::Parser)]
#[command(name = "railc")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[non_exhaustive]
enum Command {
    /// Build and execute the file
    Run {
        path: PathBuf,
    },
    Build {},
    Check {},
}

struct Driver {}

impl Driver {
    fn run(path: &Path) -> Result<()> {
        let source = Self::read(path)?;
        let syntax = Self::parse(&source)?;
        let module = Self::typecheck(syntax)?;
        let program = Self::compile(module)?;
        let _ = Self::execute(&program)?;

        Ok(())
    }

    fn read(path: &Path) -> Result<String> {
        std::fs::read_to_string(path).with_context(|| format!("can't read {}", path.display()))
    }
    fn parse(source: &str) -> Result<Syntax> {
        let lexer = Lexer::new(&source);
        let parser = Parser::new(lexer);
        parser.parse().context("parsing failed")
    }
    fn typecheck(syntax: Syntax) -> Result<Module> {
        let env = TypeEnv::new();
        let typer = Typer::new(&env);
        typer.check(syntax).context("type checking failed")
    }
    fn compile(module: Module) -> Result<Program> {
        let mut compiler = CodeGen::new();
        compiler.compile(module).context("compilation failed")
    }
    fn execute(program: &Program) -> Result<i64> {
        let mut vm = Vm::from(program);
        vm.run().context("executing failed")
    }
}

fn main() -> Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.command {
        Command::Run { path } => {
            Driver::run(&path)?;
        }
        _ => {
            unimplemented!()
        }
    };

    Ok(())
}
