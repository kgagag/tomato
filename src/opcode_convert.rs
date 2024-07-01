use log::info;

use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;


pub fn i2l(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Long(v as i64));
    frame.pc += 1;
}

pub fn i2f(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Float(v as f32));
    frame.pc += 1;
}

pub fn i2d(frame: &mut StackFrame) {
    let v = frame.popf64();
    frame.op_stack.push(StackFrameValue::Double(v as f64));
    frame.pc += 1;
}

pub fn l2i(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Int(v as i32));
    frame.pc += 1;
}

pub fn l2f(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Float(v as f32));
    frame.pc += 1;
}

pub fn l2d(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Double(v as f64));
    frame.pc += 1;
}

pub fn f2i(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Int(v as i32));
    frame.pc += 1;
}

pub fn f2l(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Long(v as i64));
    frame.pc += 1;
}

pub fn f2d(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Double(v as f64));
    frame.pc += 1;
}

pub fn d2i(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Int(v as i32));
    frame.pc += 1;
}

pub fn d2f(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Float(v as f32));
    frame.pc += 1;
}

pub fn d2l(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Long(v as i64));
    frame.pc += 1;
}

pub fn i2b(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Byte(v as i8));
    frame.pc += 1;
}

pub fn i2c(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Char(v as i16));
    frame.pc += 1;
}

pub fn i2s(frame: &mut StackFrame) {
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Short(v as i16));
    frame.pc += 1;
}
