use byteorder::LE;
use log::info;

use crate::array;
use crate::reference;
use crate::reference::reference::Reference;
use crate::runtime_data_area::create_array;
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

pub fn array_copy(method: &MethodInfo, frame: &mut StackFrame) {
    let len = frame.op_stack.pop().unwrap();
   // info!("{:?}",len);
    let des_ops = frame.op_stack.pop().unwrap();
   // info!("{:?}",des_ops);
    let des = frame.op_stack.pop().unwrap();
  //  info!("{:?}",des);
    let src_ops = frame.op_stack.pop().unwrap();
  //  info!("{:?}",src_ops);
    let src = frame.op_stack.pop().unwrap();
  //  info!("{:?}",src);
    let src_array;
    let des_array;
    let src_start;
    let des_start;
    let length;
    match src {
        StackFrameValue::Reference(reference_id) => {
            let reference = get_reference(&reference_id);
            match reference {
                Reference::Array(array) => {
                    src_array = array;
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    match des {
        StackFrameValue::Reference(reference_id) => {
            let reference = get_reference(&reference_id);
            match reference {
                Reference::Array(array) => {
                    des_array = array;
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    match src_ops {
        StackFrameValue::Int(i) => {
            src_start = i;
        }
        StackFrameValue::Byte(i) =>  {
            src_start = i as i32;
        }
        StackFrameValue::Char(i) =>  {
            src_start = i as i32;
        }
        StackFrameValue::Long(i) => {
            src_start = i as i32;
        }
        StackFrameValue::Short(i) =>  {
            src_start = i as i32;
        }
        StackFrameValue::U32(i) =>  {
            src_start = i as i32;
        }
        _=> panic!()
    }

   
    match des_ops {
        StackFrameValue::Int(i) => {
            des_start = i;
        }
        StackFrameValue::Byte(i) =>  {
            des_start = i as i32;
        }
        StackFrameValue::Char(i) =>  {
            des_start = i as i32;
        }
        StackFrameValue::Long(i) => {
            des_start = i as i32;
        }
        StackFrameValue::Short(i) =>  {
            des_start = i as i32;
        }
        StackFrameValue::U32(i) =>  {
            des_start = i as i32;
        }
        _=> panic!()
    }


    match len {
        StackFrameValue::Int(i) => {
            length = i;
        }
        StackFrameValue::Byte(i) =>  {
            length = i as i32;
        }
        StackFrameValue::Char(i) =>  {
            length = i as i32;
        }
        StackFrameValue::Long(i) => {
            length = i as i32;
        }
        StackFrameValue::Short(i) =>  {
            length = i as i32;
        }
        StackFrameValue::U32(i) =>  {
            length = i as i32;
        }
        _=> panic!()
    }

    for i in 0 .. length{
        des_array.data[(des_start  + i) as usize] = src_array.data.get((src_start + i)  as usize).unwrap().clone(); 
    }

}
