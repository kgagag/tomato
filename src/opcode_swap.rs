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

pub fn swap(frame: &mut StackFrame) {
    let last = frame.op_stack[frame.op_stack.len() - 1].clone();
    let last2 = frame.op_stack[frame.op_stack.len() - 2].clone();
    frame.op_stack.insert(frame.op_stack.len() - 1, last2);
    frame.op_stack.insert(frame.op_stack.len() - 2 , last);
    frame.pc += 1;
}