use log::info;

use crate::common::{stack_frame::StackFrame, value::StackFrameValue};


extern crate env_logger;
extern crate log;


pub fn i2l(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Long(v));
    frame.pc += 1;
}

pub fn i2f(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Float(v as f32));
    frame.pc += 1;
}

pub fn i2d(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popf64();
    frame.op_stack.push(StackFrameValue::Double(v));
    frame.pc += 1;
}

pub fn l2i(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Int(v as i32));
    frame.pc += 1;
}

pub fn l2f(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Float(v as f32));
    frame.pc += 1;
}

pub fn l2d(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Double(v as f64));
    frame.pc += 1;
}

pub fn f2i(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Int(v as i32));
    frame.pc += 1;
}

pub fn f2l(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Long(v as i64));
    frame.pc += 1;
}

pub fn f2d(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Double(v as f64));
    frame.pc += 1;
}

pub fn d2i(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Int(v as i32));
    frame.pc += 1;
}

pub fn d2f(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Float(v as f32));
    frame.pc += 1;
}

pub fn d2l(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Long(v));
    frame.pc += 1;
}

pub fn i2b(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Byte(v as i8));
    frame.pc += 1;
}

pub fn i2c(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Char(v as i16));
    frame.pc += 1;
}

pub fn i2s(vm_stack: &mut Vec<StackFrame>) {
    let frame = vm_stack.last_mut().unwrap();
    let v = frame.popi64();
    frame.op_stack.push(StackFrameValue::Short(v as i16));
    frame.pc += 1;
}
