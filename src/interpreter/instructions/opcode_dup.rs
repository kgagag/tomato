use crate::common::stack_frame::StackFrame;


extern crate log;
extern crate env_logger;


pub fn dup(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    frame.op_stack.push(frame.op_stack.last().unwrap().clone());
    frame.pc += 1;
}

pub fn dup2(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let len = frame.op_stack.len();
    let a = frame.op_stack[len - 1].clone();
    let b = frame.op_stack[len - 2].clone();
    frame.op_stack.push(b);
    frame.op_stack.push(a);
    frame.pc += 1;
}

pub fn dup_x1(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let len = frame.op_stack.len();
    let a = frame.op_stack.last().unwrap().clone();
    frame.op_stack.insert(len - 2, a);
    frame.pc += 1;
}

pub fn dup_x2(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let len = frame.op_stack.len();
    let a = frame.op_stack.last().unwrap().clone();
    frame.op_stack.insert(len - 3, a);
    frame.pc += 1;
}

pub fn dup2_x2(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2_x1(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

