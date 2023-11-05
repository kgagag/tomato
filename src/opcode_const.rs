use crate::class::ConstantPoolInfo;
use crate::class::MethodInfo;
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
use log::{error, info, warn};
use std::env;


pub fn iconst_m1(frame: &mut StackFrame) {
    frame.op_stack.push(StackFrameValue::Int(-1));
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