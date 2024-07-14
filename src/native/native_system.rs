use log::info;

use crate::{classfile::class::MethodInfo, common::{array::array::Array, reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::get_reference};


extern crate env_logger;
extern crate log;


pub fn array_copy(method: &MethodInfo, frame: &mut StackFrame) {
    let _ = method;
    let len = frame.op_stack.pop().unwrap();
    // info!("{:?}",len);
    let des_ops = frame.op_stack.pop().unwrap();
     //info!("{:?}",des_ops);
    let des = frame.op_stack.pop().unwrap();
     // info!("{:?}",des);
    let src_ops = frame.op_stack.pop().unwrap();
     // info!("{:?}",src_ops);
    let src = frame.op_stack.pop().unwrap();
     // info!("{:?}",src);

    let src_array: &mut Array = match src {
        StackFrameValue::Reference(reference_id) => {
            let reference = get_reference(&reference_id).unwrap();
            match reference {
                Reference::Array(array) => array,
                _ => panic!(),
            }
        }
        _ => panic!(),
    };
    let des_array: &mut Array = match des {
        StackFrameValue::Reference(reference_id) => {
            let reference = get_reference(&reference_id).unwrap();
            match reference {
                Reference::Array(array) => array,
                _ => panic!(),
            }
        }
        _ => panic!(),
    };

    let src_start = match src_ops {
        StackFrameValue::Int(i) => i,
        StackFrameValue::Byte(i) => i as i32,
        StackFrameValue::Char(i) => i as i32,
        StackFrameValue::Long(i) => i as i32,
        StackFrameValue::Short(i) => i as i32,
        StackFrameValue::U32(i) => i as i32,
        _ => panic!(),
    };

    let des_start = match des_ops {
        StackFrameValue::Int(i) => i ,
        StackFrameValue::Byte(i) => i as i32,
        StackFrameValue::Char(i) => i as i32,
        StackFrameValue::Long(i) => i as i32,
        StackFrameValue::Short(i) => i as i32,
        StackFrameValue::U32(i) => i as i32,
        _ => panic!(),
    };

    let length: i32 = match len {
        StackFrameValue::Int(i) => i,
        StackFrameValue::Byte(i) => i as i32,
        StackFrameValue::Char(i) => i as i32,
        StackFrameValue::Long(i) => i as i32,
        StackFrameValue::Short(i) => i as i32,
        StackFrameValue::U32(i) => i as i32,
        _ => panic!(),
    };

    for i in 0..length {
        des_array.data[(des_start + i) as usize] = src_array
            .data
            .get((src_start + i) as usize)
            .unwrap()
            .clone();
    }
}
