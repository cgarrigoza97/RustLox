use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use rlox::chunk::Chunk;
use rlox::compiler::Compiler;
use rlox::scanner::Scanner;
use rlox::vm::{self, VM};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = env::args().count();

    let chunk = Chunk::new();
    let mut scanner = Scanner::new();
    let mut compiler = Compiler::new(&mut scanner);

    let mut vm = VM::new(&chunk, &mut compiler);

    if args_count > 2 {
        eprintln!("Usage: rlox [path]");
        process::exit(64);
    } else if let Some(path) = args.get(1) {
        run_file(path, &mut vm);
    } else {
        repl(&mut vm);
    }

    process::exit(0);
}

fn repl(vm: &mut VM) {
    loop {
        let mut input = String::new();

        println!("> ");
        io::stdout().flush().unwrap();

        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if bytes_read == 0 || input.trim().is_empty() {
            println!();
            break;
        }

        vm.interpret(input);
    }
}

fn run_file(path: &str, vm: &mut VM) {
    let error_message = format!("Could not open file {}", path);
    let source = fs::read_to_string(path).expect(&error_message);

    let result = vm.interpret(source);

    if result == vm::InterpretResult::InterpretCompileError {
        process::exit(65)
    }

    if result == vm::InterpretResult::InterpretRuntimeError {
        process::exit(70)
    }
}
