use crate::{common::stack_frame::StackFrame, utils::u8c::u8s_to_u16};

extern crate log;
extern crate env_logger;

pub fn goto(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]) as i16;
    frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
}

pub fn jsr(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let _ = frame;
    
}

pub fn ret(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
   
}

