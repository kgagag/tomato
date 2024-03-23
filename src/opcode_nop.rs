
use crate::stack_frame::StackFrame;
extern crate log;
extern crate env_logger;
use log::{error, info, warn};
use std::env;

pub fn nop(frame: &mut StackFrame) {
    frame.pc += 1;
}