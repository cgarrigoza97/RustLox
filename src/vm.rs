use num_traits::FromPrimitive;

use crate::chunk::{OpCode, Chunk};
use crate::value::{self, print_value, Value};

const STACK_MAX: usize = 256;

macro_rules! binary_op {
    ($self:expr, $op:tt) => {{
        let b = $self.pop();    
        let a = $self.pop();
        $self.push(a $op b);
    }};
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError
}

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: usize,
    pub stack: [Value; STACK_MAX],
    pub stack_top: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self { 
            chunk, 
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.ip = 0;
        return self.run();
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value; 
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        return self.stack[self.stack_top];
    }
            
    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")] {
                debug::disassemble_instruction(&self.chunk, self.ip);
                print!("          ");
                for slot in 0..self.stack_top {
                    print!("[ ");
                    print_value(&self.stack[slot]);
                    print!(" ]");
                }
                println!();
            }

            let byte = self.read_byte();
            let instruction = OpCode::from_u8(byte);
            match instruction {
                Some(OpCode::OpConstant) => {
                    let constant = self.read_constant();
                    self.push(constant);
                    value::print_value(constant);
                    println!();
                }
                Some(OpCode::OpAdd) => {
                    binary_op!(self, +);
                }
                Some(OpCode::OpSubstract) => {
                    binary_op!(self, -);
                }
                Some(OpCode::OpMultiply) => {
                    binary_op!(self, *);
                }
                Some(OpCode::OpDivide) => {
                    binary_op!(self, /);
                }
                Some(OpCode::OpNegate) => {
                    let value = self.pop();
                    self.push(-value);
                }
                Some(OpCode::OpReturn) => {
                    print_value(self.pop());
                    println!();
                    return InterpretResult::InterpretOk;
                }
                _ => return InterpretResult::InterpretCompileError
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let current_ip = self.ip;
        self.ip += 1;
        return self.chunk.code[current_ip];
    }

    fn read_constant(&mut self) -> Value {
        return self.chunk.constants.values[self.read_byte() as usize];
    }

    fn reset_stack(&mut self) {
        self.stack_top = 0;
    }
}