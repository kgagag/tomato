use std::path::Component;

use clap::Id;
use log::info;

use crate::array::array::Array;
use crate::param::param::DataType;
use crate::reference::reference::Reference;
use crate::runtime_data_area::{
    create_array, create_class_object, get_class_name, get_constant_pool_class, get_reference,
    put_into_class_constant_pool,
};
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::{array, class::*, object, param, reference};

/*
 * 创建一个一维数组
 */
pub fn new_array(method: &MethodInfo, frame: &mut StackFrame) {
    let len = frame.popi64();
    let component_sfv_type_: StackFrameValue = frame.op_stack.pop().unwrap();
    let array = create_array(len as u32, DataType::Array { element_type: (Box::new(DataType::Int)), depth: (1) });
    frame.op_stack.push(StackFrameValue::Reference(array));
}

// fn get_rust_string(id: &u64) -> String {
//     let reference: &mut Reference = get_reference(&id).unwrap();
//     let mut string = String::from("");
//     match reference {
//         Reference::Object(object) => {
//             let val = object.field.get("value").unwrap();
//             match val {
//                 StackFrameValue::Reference(id) => {
//                     let reference = get_reference(id).unwrap();
//                     match reference {
//                         Reference::Array(array) => {
//                             for i in 0..array.data.len() {
//                                 let v: &StackFrameValue = array.data.get(i).unwrap();
//                                 match v {
//                                     StackFrameValue::CHARACTER(ch) => {
//                                         string.push(*ch);
//                                     }
//                                     _ => panic!(),
//                                 }
//                             }
//                         }
//                         _ => panic!(),
//                     }
//                 }
//                 _ => panic!(),
//             }
//         }
//         _ => panic!(),
//     }
//     string
// }
