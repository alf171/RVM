pub struct VM {
    memory: Vec<u8>,
    stack: Vec<i32>,
    program_counter: usize,
}

impl VM {
   pub fn new(program: Vec<u8>) -> VM {
       VM {
           memory: program,
           stack: Vec::new(),
           program_counter: 0,
       }
   }

    pub fn run(&mut self) {
        loop {
            // End of program
            if self.program_counter >= self.memory.len() {
                break;
            }

            let instruction = self.memory[self.program_counter];
            self.program_counter += 1;

            match instruction {
                0x01 => self.op_push(),
                0x02 => self.op_add(),
                0x03 => self.op_print(),
                0xFF => break,
                _ => panic!("Unknown instruction: {instruction}"),
            }
        }
    }
    fn op_push(&mut self) {
        let item = self.memory[self.program_counter] as i32;
        self.program_counter += 1;
        self.stack.push(item);
    }

    fn op_add(&mut self) {
        let item1 = self.stack.pop().unwrap();
        let item2 = self.stack.pop().unwrap();
        self.stack.push(item1 + item2);
    }
    fn op_print(&mut self) {
        let item = self.stack.pop().unwrap();
        print!("[pc: {}] printing value {item}\n", self.program_counter);
    }
}