use core::num;

use byteorder::LE;
use log::info;

use crate::array;
use crate::array::array::Array;
use crate::reference;
use crate::reference::reference::Reference;
use crate::runtime_data_area::create_array;
use crate::stack_frame;
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::class::*;
use crate::param::param::DataType;
use crate::runtime_data_area::get_class_name;
use crate::runtime_data_area::get_or_load_class;
use crate::runtime_data_area::get_reference;
use crate::u8c::u8s_to_u16;

pub fn float_to_raw_int_bits(method: &MethodInfo, frame: &mut StackFrame) {
   let f = frame.op_stack.pop().unwrap();
   match f {
       StackFrameValue::Float(ff) =>{
        frame.op_stack.push(StackFrameValue::U32(ff.to_bits()))
       }
       _=> panic!()
   }
}

pub fn int_bits_to_float(method: &MethodInfo, frame: &mut StackFrame) {
    let f = frame.op_stack.pop().unwrap();
   // info!("{:?}",f);
    match f {
        StackFrameValue::Int(ff) =>{
         frame.op_stack.push(StackFrameValue::Float(f32::from_bits(ff as u32)))
        }
        StackFrameValue::U32(ff) =>{
            frame.op_stack.push(StackFrameValue::Float(f32::from_bits(ff)))
           }
        _=> panic!()
    }
  //  info!("{:?}",frame);
 }

pub fn double_to_raw_long_bits(method: &MethodInfo, frame: &mut StackFrame) {
    let f = frame.op_stack.pop().unwrap();
    match f {
        StackFrameValue::Double(ff) =>{
         frame.op_stack.push(StackFrameValue::U64(ff.to_bits()))
        }
        _=> panic!()
    }
 }

 pub fn long_bits_to_double(method: &MethodInfo, frame: &mut StackFrame) {
    let f = frame.op_stack.pop().unwrap();
    //info!("{:?}",f);
    match f {
        StackFrameValue::Long(num) =>{
         frame.op_stack.push(StackFrameValue::Double(f64::from_bits( num as u64)))
        }
        StackFrameValue::Short(num) => {
            frame.op_stack.push(StackFrameValue::Double(f64::from_bits( num as u64)))
        },
        StackFrameValue::U32(num) => {
            frame.op_stack.push(StackFrameValue::Double(f64::from_bits( num as u64)))
        }
        StackFrameValue::U64(num) => {
            frame.op_stack.push(StackFrameValue::Double(f64::from_bits( num as u64)))
        }
        _=> panic!()
    }
 }
