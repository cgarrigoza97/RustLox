use crate::scanner::{Scanner, TokenType};

pub struct Compiler<'a> {
    pub scanner: &'a mut Scanner,
}

impl<'a> Compiler<'a> {
    pub fn new(scanner: &'a mut Scanner) -> Self {
        Self { scanner }
    }

    pub fn compile(&mut self, source: String) {
        let mut line = -1;
        self.scanner.set_source(source);
        loop {
            let token = self.scanner.scan_token();

            if token.line != line {
                print!("{} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }

            let token_value = self.scanner.get_token_value(&token);
            println!("{:2} '{}'", token.token_type as u8, token_value);

            if token.token_type == TokenType::Eof {
                break;
            }
        }
    }
}
