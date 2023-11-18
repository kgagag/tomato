
use crate::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;
use log::{error, info, warn};

pub fn dup(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup_x1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup_x2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2_x2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2_x1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

