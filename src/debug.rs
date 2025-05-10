use crate::chunk::{OpCode, Chunk};
use crate::value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.chunk_len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    let line_number = chunk.get_line(offset);
    print!("{:04} ", line_number);


    let instruction = chunk.code[offset];
    let instruction_op_code = num::FromPrimitive::from_u8(instruction);
    match instruction_op_code {
        Some(OpCode::OpConstant) => return constant_instruction("OP_CONSTANT", &chunk, offset),
        Some(OpCode::OpReturn) => return simple_instruction("OP_RETURN", offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            return offset + 1;
        }
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;

    print!("{:<16} {:>4} '", name, constant);
    value::print_value(chunk.constants.values[constant]);
    println!("");
    
    return offset + 2;
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}