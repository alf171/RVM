use crate::vm::VM;

mod vm;

// 0x01 => self.op_push(),
// 0x02 => self.op_add(),
// 0x03 => self.op_print(),
fn main() {
    let program = vec![0x01, 0x02, 0x01, 0x02, 0x02, 0x06];
    let mut vm = VM::new(program);
    vm.run();
}