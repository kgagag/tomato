use crate::common::stack_frame::StackFrame;


extern crate log;
extern crate env_logger;


pub fn dup(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack.peek());
    frame.pc += 1;
}

pub fn dup2(frame: &mut StackFrame) {
    let len = frame.op_stack.len();
    let a = frame.op_stack.get(len - 1);
    let b = frame.op_stack.get(len - 2);
    frame.op_stack.push(b);
    frame.op_stack.push(a);
    frame.pc += 1;
}

pub fn dup_x1(frame: &mut StackFrame) {
    // let len = frame.op_stack.len();
    // let a = frame.op_stack.last().unwrap().clone();
    // frame.op_stack.insert(len - 2, a);
    // frame.pc += 1;
}

pub fn dup_x2(frame: &mut StackFrame) {
    // let len = frame.op_stack.len();
    // let a = frame.op_stack.last().unwrap().clone();
    // frame.op_stack.insert(len - 3, a);
    // frame.pc += 1;
}

pub fn dup2_x2(frame: &mut StackFrame) {
    // frame.op_stack.push(frame.op_stack[0].clone());
    // frame.pc += 1;
}

pub fn dup2_x1(frame: &mut StackFrame) {
    // frame.op_stack.push(frame.op_stack[0].clone());
    // frame.pc += 1;
}

