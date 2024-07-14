
use crate::common::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;

pub fn athrow(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack.last().unwrap().clone());
    frame.pc += 1;
}