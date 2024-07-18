use core::num;

use byteorder::LE;
use log::info;

use crate::{classfile::class::MethodInfo, common::{stack_frame::StackFrame, value::StackFrameValue}};


pub fn float_to_raw_int_bits(frame: &mut StackFrame) {
   let f = frame.op_stack.pop().unwrap();
   match f {
       StackFrameValue::Float(ff) =>{
        frame.op_stack.push(StackFrameValue::U32(ff.to_bits()))
       }
       _=> panic!()
   }
}

pub fn int_bits_to_float(frame: &mut StackFrame) {
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

pub fn double_to_raw_long_bits( frame: &mut StackFrame) {
    let f = frame.op_stack.pop().unwrap();
    match f {
        StackFrameValue::Double(ff) =>{
         frame.op_stack.push(StackFrameValue::U64(ff.to_bits()))
        }
        _=> panic!()
    }
 }

 pub fn long_bits_to_double( frame: &mut StackFrame) {
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
            frame.op_stack.push(StackFrameValue::Double(f64::from_bits( num )))
        }
        _=> panic!()
    }
 }
