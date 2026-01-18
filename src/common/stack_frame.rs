
use log::info;
use std::cell::UnsafeCell;
use std::collections::HashMap;

use crate::{classfile::class::{AttributeInfo, Class, CodeAttribute, ConstantPoolInfo, MethodInfo}, common::op_stack::OpStack, utils::u8c::u8s_to_u16};

use super::{param::DataType, value::{number_to_u32tuple, StackFrameValue}};
/**
 * 栈桢
 */
#[derive(Debug, Clone)]
pub struct StackFrame {
    // //指令
    // pub code: Vec<u8>,
    //程序计数器
    pub pc: usize,
    // //局部变量表
    pub local: Vec<StackFrameValue>,
    // //操作数栈
    pub op_stack : Vec<StackFrameValue>,
    // //类
    pub class: usize,

    pub max_stack: u16,

    pub max_locals: u16,

    pub code: Vec<u8>,

    //pub code_attr: CodeAttribute,

    //所属虚拟机栈id
    pub vm_stack_id: u32,

    pub method_name:String,

    pub descriptor:String,

    pub class_name:String
}

impl StackFrame {
    pub fn new(
        class: usize,
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        //code_attr: CodeAttribute,
        method_name:String,
        descriptor:String,
        class_name:String
    ) -> StackFrame {
        let mut stake_frame = StackFrame {
            pc: 0,
            class,
            local: Vec::new(),
            op_stack: Vec::new(),
            max_stack,
            max_locals,
            code,
            //code_attr,
            vm_stack_id: 0,
            method_name,
            descriptor,
            class_name
        };
        for _i in 0..stake_frame.max_locals as usize {
            stake_frame.local.push(StackFrameValue::Byte(0));
        }
        //info!("{:?}",stake_frame);
        stake_frame
    }

    pub fn popi64(&mut self) -> i64 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Int(data) => data as i64,
            StackFrameValue::Byte(data) => data as i64,
            StackFrameValue::Char(data) => data as i64,
            StackFrameValue::Long(data) => data ,
            StackFrameValue::Short(data) => data as i64,
            StackFrameValue::U32(data) => data as i64,
            StackFrameValue::CHARACTER(data) => data as i64,
            StackFrameValue::Boolean(data) => data as i64,
            StackFrameValue::Double(data) => data as i64,
            StackFrameValue::Float(data) => data as i64,
            _ => {
                panic!("wrong value type:{:?}",value);
            }
        }
    }

    pub fn popf64(&mut self) -> f64 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Double(data) => data ,
            StackFrameValue::Float(data) => data as f64,
            _ => {
                panic!("wrong value type:{:?}",value);
            }
        }
    }
}

pub fn init_stack_frame(
    frame: &mut StackFrame,
    method_info: &MethodInfo,
    start: usize,
) -> StackFrame {
    let mut new_stack_frame: StackFrame = create_stack_frame(method_info).unwrap();
    new_stack_frame.vm_stack_id = frame.vm_stack_id;
    let mut i: usize = start;
    let op_stack_len = frame.op_stack.len();
    let param_len =  method_info.param.len();
    if !method_info.param.is_empty()  {
        for j in 0..method_info.param.len() {
            let v = frame.op_stack[op_stack_len - param_len + j];
            let param: &DataType = method_info.param.get(j).unwrap();
            match param {
                DataType::Byte => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Char => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Array {
                    element_type: _,
                    depth: _,
                } => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Boolean => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Double => {
                    let u32tuple: (u32, u32) = number_to_u32tuple(&v);
                    new_stack_frame.local[i] = StackFrameValue::U32(u32tuple.0);
                    new_stack_frame.local[i + 1] = StackFrameValue::U32(u32tuple.1);
                    i += 2;
                }
                DataType::Float => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Int => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Long => {
                    let u32tuple = number_to_u32tuple(&v);
                    new_stack_frame.local[i] = StackFrameValue::U32(u32tuple.0);
                    new_stack_frame.local[i + 1] = StackFrameValue::U32(u32tuple.1);
                    i += 2;
                }
                DataType::Reference(_string) => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Short => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                _ => panic!(),
            }
        }
    }

    for _j in 0..method_info.param.len() {
        _=  frame.op_stack.pop()
    }

    new_stack_frame
}

pub fn create_stack_frame(method_info: &MethodInfo) -> Option<StackFrame> {
    for attr in &method_info.attributes {
        match attr {
            AttributeInfo::Code(code_attr) => {
                return Some(StackFrame::new(
                    method_info.class_id as usize,
                    code_attr.max_stack,
                    code_attr.max_locals,
                    code_attr.code.clone(),
                   // code_attr.clone(),
                    method_info.method_name.clone(),
                    method_info.descriptor.clone(),
                    method_info.class_name.clone()
                ))
            }
        }
    }
    None
}

pub fn update_stack_frame(frame: &mut StackFrame, method_info: &MethodInfo) {
    for attr in &method_info.attributes {
        match attr {
            AttributeInfo::Code(code_attr) => {
                frame.max_stack = code_attr.max_stack;
                frame.max_locals = code_attr.max_locals;
                frame.code = code_attr.code.clone();
                frame.class_name = method_info.class_name.clone();
                frame.class = method_info.class_id as usize;
            }
        }
    }
}