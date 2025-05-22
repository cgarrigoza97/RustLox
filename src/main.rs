use std::env;
use std::process;

use rlox::chunk::{Chunk, OpCode};
use rlox::vm::VM;
use rlox::debug;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut chunk = Chunk::new();

    let mut constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant, 123);
    constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::OpDivide as u8, 123);

    chunk.write_chunk(OpCode::OpNegate as u8, 123);
    chunk.write_chunk(OpCode::OpReturn as u8, 123);
    
    debug::disassemble_chunk(&chunk, "test chunk");

    let mut vm = VM::new(&chunk);
    let result = vm.interpret();

    process::exit(0);
}
