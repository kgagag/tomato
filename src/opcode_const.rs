
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate log;
extern crate env_logger;
use log::*;

pub fn iconst_m1(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(1));
    frame.pc += 1;
}

pub fn iconst_0(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(0));
    frame.pc += 1;
}

pub fn iconst_1(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(1));
    frame.pc += 1;
}

pub fn iconst_2(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(2));
    frame.pc += 1;
}


pub fn iconst_3(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(3));
    frame.pc += 1;
}

pub fn iconst_4(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(4));
    frame.pc += 1;
}

pub fn iconst_5(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(5));
    frame.pc += 1;
}

pub fn iconst_ml(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(-1));
    frame.pc += 1;
}

pub fn aconst_null(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Null);
    frame.pc += 1;
}

pub fn lconst_0(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Long(0));
    frame.pc += 1;
}

pub fn lconst_1(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Long(1));
    frame.pc += 1;
}

pub fn fconst_0(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Float(0.0));
    frame.pc += 1;
}

pub fn fconst_1(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Float(1.0));
    frame.pc += 1;
}

pub fn fconst_2(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Float(2.0));
    frame.pc += 1;
}

pub fn dconst_0(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Double(0.0));
    frame.pc += 1;
}

pub fn dconst_1(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Double(0.0));
    frame.pc += 1;
}