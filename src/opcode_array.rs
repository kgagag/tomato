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



pub fn newarray(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    //info!("{:?}",v);
    let atype = frame.code.get(frame.pc + 1).unwrap();
    //info!("{:?}",atype);
    let reference ;
    let array_type;
    let mut len:i32 = 0;
    match v {
        StackFrameValue::Byte(l) => len = l as i32,
        StackFrameValue::Char(l) => len = l as i32,
        StackFrameValue::Int(l) => len = l as i32,
        StackFrameValue::Short(l) => len = l as i32,
        _=> panic!()
    }
    if *atype == 4{
        array_type =  MethodParameter::Boolean;
       }else if  *atype == 5{
        array_type =  MethodParameter::Char;
       }else if  *atype == 6{
        array_type =  MethodParameter::Float;
       } else if  *atype == 7{
        array_type =  MethodParameter::Double;
       } else if  *atype == 8{
        array_type =  MethodParameter::Byte;
       } else if  *atype == 9{
        array_type =  MethodParameter::Short;
       }else if  *atype == 10{
        array_type =  MethodParameter::Int;
       }else if  *atype == 11{
        array_type =  MethodParameter::Long;
       }else{
        panic!("wrong atype");
       } 
      reference = create_array(len as u32, array_type);
      match reference{
        Reference::Array(arr) =>{
            frame.op_stack.push(StackFrameValue::Reference(arr.id));
        }
        _=> panic!(),
      }

    frame.pc += 2;
}

pub fn iastore(frame: &mut StackFrame){
    xastore(frame);
}

pub fn lastore(frame: &mut StackFrame){
    xastore(frame);
}
pub fn fastore(frame: &mut StackFrame){
    xastore(frame);
}

pub fn dastore(frame: &mut StackFrame){
    xastore(frame);
}

pub fn aastore(frame: &mut StackFrame){
    xastore(frame);
}

pub fn bastore(frame: &mut StackFrame){
    xastore(frame);
}

pub fn castore(frame: &mut StackFrame){
    xastore(frame);
}

pub fn sastore(frame: &mut StackFrame){
    xastore(frame);
}

fn xastore(frame: &mut StackFrame){
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    let i;
    match index {
        StackFrameValue::Byte(l) => i = l as usize,
        StackFrameValue::Char(l) => i = l as usize,
        StackFrameValue::Int(l) => i = l as usize,
        StackFrameValue::Short(l) => i = l as usize,
        _=> panic!()
    }
    match array {
        StackFrameValue::Reference(reference_id) =>{
            let reference: &mut Reference = get_reference(&reference_id);
            info!("{:?}",reference);
            match reference {
                Reference::Array(arr) =>{
                    arr.data.insert(i, v);
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.pc += 1;
}

pub fn iaload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn laload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn faload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn daload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn aaload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn baload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn caload(frame: &mut StackFrame){
    xaload(frame);
}

pub fn saload(frame: &mut StackFrame){
    xaload(frame);
}

fn xaload(frame: &mut StackFrame){
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    let i;
    match index {
        StackFrameValue::Byte(l) => i = l as usize,
        StackFrameValue::Char(l) => i = l as usize,
        StackFrameValue::Int(l) => i = l as usize,
        StackFrameValue::Short(l) => i = l as usize,
        _=> panic!()
    }
    match array {
        StackFrameValue::Reference(reference_id) =>{
            let reference = get_reference(&reference_id);
            info!("{:?}",reference);
            match reference {
                Reference::Array(arr) =>{
                   frame.op_stack.push(arr.data.get(i as usize).unwrap().clone());
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.pc += 1;
}

