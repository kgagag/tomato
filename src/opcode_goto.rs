
use crate::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;
use log::{error, info, warn};
use crate::u8c::*;

pub fn goto(frame: &mut StackFrame) {
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]) as i16;
    frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
}

pub fn jsr(frame: &mut StackFrame) {
    
}

pub fn ret(frame: &mut StackFrame) {
   
}

