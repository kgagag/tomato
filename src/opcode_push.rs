use crate::stack_frame::StackFrame;
use crate::stack_frame::init_stack_frame;
use crate::value::value::StackFrameValue;
extern crate log;
extern crate env_logger;
use crate::param::param::DataType;
use log::{error, info, warn};
use std::env;
use crate::u8c::u8s_to_u16;
pub fn bipush(frame: &mut StackFrame) {
    let u = frame.code[frame.pc + 1];
    frame.op_stack.push(StackFrameValue::Byte(u as i8));
    frame.pc += 2;
}

pub fn sipush(frame: &mut StackFrame) {
    frame
        .op_stack
        .push(StackFrameValue::Short(u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]) as i16));
    frame.pc += 3;
}