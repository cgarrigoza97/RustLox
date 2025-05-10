use num_derive::FromPrimitive;

use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(FromPrimitive)]
pub enum OpCode {
    OpConstant,
    OpReturn
}

pub struct LineEncoding {
    pub line_number: i32,
    pub repetition_count: i32
}

impl LineEncoding {
    pub fn new(line: i32) -> Self {
        Self { line_number: line, repetition_count: 1 }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<LineEncoding>
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
        
        if let Some(last) = self.lines.last_mut() {
            if last.line_number == line {
                last.repetition_count += 1;
                return;
            }
        }

        self.lines.push(LineEncoding::new(line));
    }
    
    pub fn get_line(&self, code_index: usize) -> i32 {
        let mut total_code_count: usize = 0;
    
        for i in 0..self.lines.len() {
            total_code_count += self.lines[i].repetition_count as usize;

            if code_index < total_code_count {
                return self.lines[i].line_number;
            }
        }

        return 0;
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write_value_array(value);
        return (self.constants.len() - 1) as u8;
    }

    pub fn chunk_len(&self) -> usize {
        self.code.len()
    }
}