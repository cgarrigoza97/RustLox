pub type Value = f64;

pub struct ValueArray {
    pub values: Vec<Value>
}

impl ValueArray {
    pub fn new() -> Self {
        Self { 
            values: Vec::new() 
        }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

pub fn print_value(value: Value) {
    print!("{:}", value);
}