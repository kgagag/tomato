use crate::common::stack_frame::StackFrame;


extern crate log;
extern crate env_logger;


pub fn monitorenter(frame: &mut StackFrame) {
    frame.pc += 1;
}


pub fn monitorexit(frame: &mut StackFrame) {
    frame.pc += 1;
}