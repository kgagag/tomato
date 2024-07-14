
use crate::common::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;


pub fn nop(frame: &mut StackFrame) {
    frame.pc += 1;
}