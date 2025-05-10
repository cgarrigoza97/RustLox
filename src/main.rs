use std::env;
use std::process;

use rlox::chunk::{Chunk, OpCode};
use rlox::debug;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant, 123);
    chunk.write_chunk(OpCode::OpReturn as u8, 123);
    
    debug::disassemble_chunk(&chunk, "test chunk");

    process::exit(0);
}
