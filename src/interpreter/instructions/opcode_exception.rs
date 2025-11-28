
use crate::common::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;

pub fn athrow(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    frame.op_stack.push(frame.op_stack.last().unwrap().clone());
    frame.pc += 1;
}