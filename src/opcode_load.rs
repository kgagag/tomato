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
use crate::u8c::u8s_to_u64;

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
pub fn aload_1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn aload_2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(2).unwrap().clone());
    frame.pc += 1;
}

pub fn aload_3(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(3).unwrap().clone());
    frame.pc += 1;
}

pub fn aload_0(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}

pub fn iload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local.get(index).unwrap().clone());
    frame.pc += 2;
}


pub fn fload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local.get(index).unwrap().clone());
    frame.pc += 2;
}

pub fn aload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local.get(index).unwrap().clone());
    frame.pc += 2;
}


pub fn dload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    let u1 = frame.local.get(index).unwrap().clone();
    let u2  =  frame.local.get(index + 1).unwrap().clone();
    let mut bytes1:[u8;4]=[0,0,0,0];
    let mut bytes2:[u8;4]=[0,0,0,0];
    match u1 {
        StackFrameValue::Byte(l) => bytes1 = (l as u32).to_be_bytes(),
        StackFrameValue::Char(l) => bytes1 = (l as u32).to_be_bytes(),
        StackFrameValue::Int(l) =>  bytes1 = (l as u32).to_be_bytes(),
        StackFrameValue::Short(l) => bytes1 = (l as u32).to_be_bytes(),
        _=> panic!()
    }
    match u2 {
        StackFrameValue::Byte(l) => bytes2 = (l as u32).to_be_bytes(),
        StackFrameValue::Char(l) => bytes2 = (l as u32).to_be_bytes(),
        StackFrameValue::Int(l) =>  bytes2 = (l as u32).to_be_bytes(),
        StackFrameValue::Short(l) => bytes2 = (l as u32).to_be_bytes(),
        _=> panic!()
    }
    let bytes3 = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
    frame.op_stack.push(StackFrameValue::Double(u8s_to_u64(&bytes3) as f64));
    frame.pc += 2;
}

pub fn lload(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as usize;
    let u1 = frame.local.get(index).unwrap().clone();
    let u2  =  frame.local.get(index + 1).unwrap().clone();
    let mut bytes1:[u8;4]=[0,0,0,0];
    let mut bytes2:[u8;4]=[0,0,0,0];
    match u1 {
        StackFrameValue::Byte(l) => bytes1 = (l as u32).to_be_bytes(),
        StackFrameValue::Char(l) => bytes1 = (l as u32).to_be_bytes(),
        StackFrameValue::Int(l) =>  bytes1 = (l as u32).to_be_bytes(),
        StackFrameValue::Short(l) => bytes1 = (l as u32).to_be_bytes(),
        _=> panic!()
    }
    match u2 {
        StackFrameValue::Byte(l) => bytes2 = (l as u32).to_be_bytes(),
        StackFrameValue::Char(l) => bytes2 = (l as u32).to_be_bytes(),
        StackFrameValue::Int(l) =>  bytes2 = (l as u32).to_be_bytes(),
        StackFrameValue::Short(l) => bytes2 = (l as u32).to_be_bytes(),
        _=> panic!()
    }
    let bytes3 = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
    frame.op_stack.push(StackFrameValue::Long(u8s_to_u64(&bytes3) as i64));
    frame.pc += 2;

}
pub fn iload_0(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn iload_1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn iload_2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(2).unwrap().clone());
    frame.pc += 1;
}

pub fn iload_3(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(3).unwrap().clone());
    frame.pc += 1;
}


pub fn fload_0(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn fload_1(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn fload_2(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(2).unwrap().clone());
    frame.pc += 1;
}

pub fn fload_3(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(3).unwrap().clone());
    frame.pc += 1;
}

pub fn lload_0(frame: &mut StackFrame) {
    let v1 = frame.local.get(0).unwrap().clone();
    let v2 = frame.local.get(1).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Long(u8s_to_u64(&bytes3) as i64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn lload_1(frame: &mut StackFrame) {
    let v1 = frame.local.get(1).unwrap().clone();
    let v2 = frame.local.get(2).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Long(u8s_to_u64(&bytes3) as i64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn lload_2(frame: &mut StackFrame) {
    let v1 = frame.local.get(2).unwrap().clone();
    let v2 = frame.local.get(3).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Long(u8s_to_u64(&bytes3) as i64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn lload_3(frame: &mut StackFrame) {
    let v1 = frame.local.get(3).unwrap().clone();
    let v2 = frame.local.get(4).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Long(u8s_to_u64(&bytes3) as i64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}


pub fn dload_0(frame: &mut StackFrame) {
    let v1 = frame.local.get(0).unwrap().clone();
    let v2 = frame.local.get(1).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Double(u8s_to_u64(&bytes3) as f64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.op_stack.push(frame.local.get(0).unwrap().clone());
    frame.pc += 1;
}


pub fn dload_1(frame: &mut StackFrame) {
    let v1 = frame.local.get(1).unwrap().clone();
    let v2 = frame.local.get(2).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Double(u8s_to_u64(&bytes3) as f64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn dload_2(frame: &mut StackFrame) {
    let v1 = frame.local.get(2).unwrap().clone();
    let v2 = frame.local.get(3).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Double(u8s_to_u64(&bytes3) as f64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

pub fn dload_3(frame: &mut StackFrame) {
    let v1 = frame.local.get(3).unwrap().clone();
    let v2 = frame.local.get(4).unwrap().clone();
    match v1 {
        StackFrameValue::U32(u1) =>{
            match v2 {
                StackFrameValue::U32(u2) =>{
                    let bytes1: [u8; 4] = unsafe { std::mem::transmute(u1) };
                    let bytes2: [u8; 4] = unsafe { std::mem::transmute(u2) };
                    let bytes3: [u8; 8] = [bytes1[0],bytes1[1],bytes1[2],bytes1[3],bytes2[0],bytes2[1],bytes2[2],bytes2[3]];
                    frame.op_stack.push(StackFrameValue::Double(u8s_to_u64(&bytes3) as f64));
                } ,
                _=> panic!()
            }
        }
        _=> panic!()
    }
}


