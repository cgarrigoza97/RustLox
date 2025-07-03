use crate::chunk::{Chunk, OpCode};
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
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:04} ", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];
    let instruction_op_code = num::FromPrimitive::from_u8(instruction);
    match instruction_op_code {
        Some(OpCode::OpConstant) => return constant_instruction("OP_CONSTANT", &chunk, offset),
        Some(OpCode::OpAdd) => return simple_instruction("OP_ADD", offset),
        Some(OpCode::OpSubstract) => return simple_instruction("OP_SUBSTRACT", offset),
        Some(OpCode::OpMultiply) => return simple_instruction("OP_MULTIPLY", offset),
        Some(OpCode::OpDivide) => return simple_instruction("OP_DIVIDE", offset),
        Some(OpCode::OpNegate) => return simple_instruction("OP_NEGATE", offset),
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
    println!();

    return offset + 2;
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}
