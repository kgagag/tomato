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

pub fn dup(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup_x1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup_x2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2_x2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

pub fn dup2_x1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.op_stack[0].clone());
    frame.pc += 1;
}

