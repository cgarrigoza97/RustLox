use num_derive::FromPrimitive;

use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(FromPrimitive)]
pub enum OpCode {
    OpConstant,
    OpReturn
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<i32>
}

impl Chunk {
    pub fn new() -> Self {
        Self { 
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new()
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write_value_array(value);
        return (self.constants.len() - 1) as u8;
    }

    pub fn chunk_len(&self) -> usize {
        self.code.len()
    }
}