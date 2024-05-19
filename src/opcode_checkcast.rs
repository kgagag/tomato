use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::u8c::*;

pub fn checkcast(frame: &mut StackFrame) {
    frame.pc += 3;
}
