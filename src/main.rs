use rail::bytecode::OpCode;
use rail::bytecode::chunk::Chunk;
use rail::bytecode::disassembler::disassemble;
use rail::runtime::function::Function;
use rail::runtime::program::Program;
use rail::vm::vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_int64(6, 0);
    chunk.add_int64(7, 1);
    chunk.add_instruction(OpCode::I64Mul, 3);
    chunk.add_instruction(OpCode::Return, 3);

    let dis = disassemble(&chunk);
    dis.iter().for_each(|string| println!("{string}"));
    println!();

    let main_fn = Function {
        name: "main".to_string(),
        chunk,
        arity: 0,
    };

    let mut program = Program::new();
    program.functions.push(main_fn);

    let mut vm = VM::from(&program);
    let e = vm.run();
    println!("{e:?}")
}
