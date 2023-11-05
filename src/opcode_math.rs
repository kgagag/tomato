use crate::class::ConstantPoolInfo;
use crate::class::MethodInfo;
use crate::reference;
use crate::reference::reference::Reference;
use crate::runtime_data_area::VM_STACKS;
use crate::runtime_data_area::create_array;
use crate::runtime_data_area::pop_stack_frame;
use crate::runtime_data_area::push_frame_data;
use crate::stack_frame::StackFrame;
use crate::stack_frame::init_stack_frame;
use crate::u8c::f64_to_u32_vec;
use crate::u8c::i64_to_u32_vec;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use crate::value::value::StackFrameValue;
use crate::runtime_data_area::get_class_name;
use crate::runtime_data_area::get_or_load_class;
use crate::runtime_data_area::create_object;
use crate::runtime_data_area::get_method_from_pool;
extern crate log;
extern crate env_logger;
use crate::param::param::MethodParameter;
use crate::runtime_data_area::get_reference;
use log::{error, info, warn};
use std::env;


pub fn iadd(frame: &mut StackFrame) {
    let i1 = frame.popu64() as i32;
    let i2 = frame.popu64() as i32;
    let result = i1 + i2;
    warn!("{}", format!("{}{}", "iadd add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn fadd(frame: &mut StackFrame) {
    let i1 = frame.popu64() as f32;
    let i2 = frame.popu64() as f32;
    let result = i1 + i2;
    warn!("{}", format!("{}{}", "fadd add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn dadd(frame: &mut StackFrame) {
    let i1 = frame.popu64() as f64;
    let i2 = frame.popu64() as f64;
    let result = i1 + i2;
    warn!("{}", format!("{}{}", "fadd add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}


pub fn ladd(frame: &mut StackFrame) {
    let i1 = frame.popu64() as i64;
    let i2 = frame.popu64() as i64;
    let result = i1 + i2;
    warn!("{}", format!("{}{}", "ladd add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}

pub fn isub(frame: &mut StackFrame) {
    let i2 = frame.popu64() as i32;
    let i1 = frame.popu64() as i32;
    let result = i1 - i2;
    warn!("{}", format!("{}{}", "isub add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn fsub(frame: &mut StackFrame) {
    let f2 = frame.popu64() as f32;
    let f1 = frame.popu64() as f32;
    let result = f1 - f2;
    warn!("{}", format!("{}{}", "isub add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn dsub(frame: &mut StackFrame) {
    let d2 = frame.popu64() as f64;
    let d1 = frame.popu64() as f64;
    let result = d1 - d2;
    warn!("{}", format!("{}{}", "dsub add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

pub fn lsub(frame: &mut StackFrame) {
    let l2 = frame.popu64() as i64;
    let l1 = frame.popu64() as i64;
    let result = l1 - l2;
    warn!("{}", format!("{}{}", "lsub add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}

pub fn fmul(frame: &mut StackFrame) {
    let f2 = frame.popu64() as f32;
    let f1 = frame.popu64() as f32;
    let result = f1 - f2;
    warn!("{}", format!("{}{}", "fmul add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn imul(frame: &mut StackFrame) {
    let i2 = frame.popu64() as i32;
    let i1 = frame.popu64() as i32;
    let result = i1 - i2;
    warn!("{}", format!("{}{}", "imul add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn lmul(frame: &mut StackFrame) {
    let l2 = frame.popu64() as i64;
    let l1 = frame.popu64() as i64;
    let result = l1 - l2;
    warn!("{}", format!("{}{}", "lmul add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}

pub fn dmul(frame: &mut StackFrame) {
    let d2 = frame.popu64() as f64;
    let d1 = frame.popu64() as f64;
    let result = d1 - d2;
    warn!("{}", format!("{}{}", "dmul add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}


pub fn idiv(frame: &mut StackFrame) {
    let i2 = frame.popu64() as i32;
    let i1 = frame.popu64() as i32;
    if i2 == 0 {
        panic!()
    }
    let result = i1 / i2;
    warn!("{}", format!("{}{}", "idiv add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn fdiv(frame: &mut StackFrame) {
    let f2 = frame.popu64() as f32;
    let f1 = frame.popu64() as f32;
    if f2 == 0.0 {
        panic!()
    }
    let result = f1 / f2;
    warn!("{}", format!("{}{}", "fdiv add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn ddiv(frame: &mut StackFrame) {
    let d2 = frame.popu64() as f64;
    let d1 = frame.popu64() as f64;
    if d2 == 0.0 {
        panic!()
    }
    let result = d1 - d2;
    warn!("{}", format!("{}{}", "ddiv add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

pub fn ldiv(frame: &mut StackFrame) {
    let l2 = frame.popu64() as i64;
    let l1 = frame.popu64() as i64;
    if l2 == 0 {
        panic!()
    }
    let result = l1 / l2;
    warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}
