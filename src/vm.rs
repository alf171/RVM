pub struct VM {
    // store instructions
    memory: Vec<u8>,
    // data stack for values
    stack: Vec<i32>,
    // call stack for return addresses
    call_stack: Vec<usize>,
    // current frame pointer
    frame_pointer: usize,
    // where in memory we are reading from currently
    program_counter: usize,
}

impl VM {
   pub fn new(program: Vec<u8>) -> VM {
       VM {
           memory: program,
           stack: Vec::new(),
           call_stack: Vec::new(),
           frame_pointer: 0,
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
                0x03 => self.op_sub(),
                0x04 => self.op_mul(),
                0x05 => self.op_div(),
                0x06 => self.op_print(),

                // function call intrinsics
                0x10 => self.op_call(),
                0x11 => self.op_return(),
                0x12 => self.op_jump(),
                0x13 => self.of_jump_if(),

                // frame instructions
                0x20 => self.op_store_local(),
                0x21 => self.op_load_local(),
                0x22 => self.op_store_param(),
                0x23 => self.op_load_param(),

                0xFF => break,
                _ => panic!("Unknown instruction: {instruction}"),
            }
        }
    }

    // push item onto the stack
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

    fn op_sub(&mut self) {
        let item1 = self.stack.pop().unwrap();
        let item2 = self.stack.pop().unwrap();
        self.stack.push(item1 - item2);
    }

    fn op_mul(&mut self) {
        let item1 = self.stack.pop().unwrap();
        let item2 = self.stack.pop().unwrap();
        self.stack.push(item1 * item2);
    }

    fn op_div(&mut self) {
        let item1 = self.stack.pop().unwrap();
        let item2 = self.stack.pop().unwrap();
        self.stack.push(item1 / item2);
    }

    fn op_print(&mut self) {
        let item = self.stack.pop().unwrap();
        print!("[pc: {}] printing value {item}\n", self.program_counter);
    }

    // enter function
    fn op_call(&mut self) {
        // read the function address off the stack
        let function_address = self.stack[self.program_counter] as usize;
        self.program_counter += 1;
        // save the current pc to the call stack
        self.call_stack.push(self.program_counter);
        // save the current fp to the call stack
        self.call_stack.push(self.frame_pointer);

        // jump to the new function
        self.program_counter = function_address;

        // set the new frame pointer to current stack
        self.frame_pointer = self.stack.len();

    }

    // leave function -- return value is on the top of the stack
    fn op_return(&mut self) {
        // restore the previous frame pointer
        self.frame_pointer = self.call_stack.pop().unwrap();
        // restore the previous program counter
        self.program_counter = self.call_stack.pop().unwrap();
    }

    // jump to address
    fn op_jump(&mut self) {
        self.program_counter = self.memory[self.program_counter] as usize;
    }

    fn of_jump_if(&mut self) {
        let condition = self.stack.pop().unwrap();
        let address = self.memory[self.program_counter] as usize;
        self.program_counter += 1;
        if condition != 0 {
            self.program_counter = address
        }
    }

     /// Stores a value from the stack into a local variable.
     /// Local variables are stored at positive offsets from the frame pointer.
    fn op_store_local(&mut self) {
        let index = self.memory[self.program_counter] as usize;
        self.program_counter += 1;

        let value = self.stack.pop().unwrap();

        let new_position = self.frame_pointer + index;

        if new_position >= self.stack.len() {
            panic!("Out of bounds load parameter: {new_position}");
        }

        self.stack[new_position] = value;
    }
    /// Load a local variable onto the stack.
    /// Local variables are accessed at positive offsets from the frame pointer
    fn op_load_local(&mut self) {
        let index = self.memory[self.program_counter] as usize;
        self.program_counter += 1;

        let new_position = self.frame_pointer + index;

        if new_position >= self.stack.len() {
            panic!("Out of bounds load parameter: {new_position}");
        }

        let value = self.stack[new_position];
        self.stack.push(value);
    }
    /// Store a value from the stack to the function parameters slot.
    /// Parameters are stored at negative offset from the frame pointer.
    fn op_store_param(&mut self) {
        let index = self.memory[self.program_counter] as usize;
        self.program_counter += 1;

        let value = self.stack.pop().unwrap();

        let new_position = self.frame_pointer - (index + 1);

        if new_position >= self.stack.len() {
            panic!("Out of bounds store parameter: {new_position}");
        }

        self.stack[new_position] = value;
    }
    /// Loads a function parameter value onto the stack.
    /// Parameters are accessed at negative offset from the frame pointer
    fn op_load_param(&mut self) {
        let index = self.memory[self.program_counter] as usize;
        self.program_counter += 1;

        let new_position = self.frame_pointer - (index + 1);

        if new_position >= self.stack.len() {
            panic!("Out of bounds store parameter: {new_position}");
        }

        let value = self.stack[new_position];
        self.stack.push(value);
    }
}
