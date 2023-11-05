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