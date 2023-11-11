
use crate::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;
use log::{error, info, warn};
use std::env;

pub fn swap(frame: &mut StackFrame) {
    let last = frame.op_stack[frame.op_stack.len() - 1].clone();
    let last2 = frame.op_stack[frame.op_stack.len() - 2].clone();
    frame.op_stack.insert(frame.op_stack.len() - 1, last2);
    frame.op_stack.insert(frame.op_stack.len() - 2 , last);
    frame.pc += 1;
}