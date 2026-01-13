
use log::info;

use crate::{classfile::class::MethodInfo, common::{param::DataType, stack_frame::StackFrame, value::StackFrameValue}, runtime::{heap::Heap, metaspace::Metaspace}};



/*
 * 创建一个一维数组
 */
pub fn new_array(method: &MethodInfo, vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let _ = method;
    let len = frame.popi64();
    let _component_sfv_type_: StackFrameValue = frame.op_stack.pop().unwrap();
    //let array = create_array(len as u32, DataType::Array { element_type: (Box::new(DataType::Int)), depth: (1) });
    let id = heap.create_basic_array(10, len as u32, 1);
    frame.op_stack.push(StackFrameValue::Reference(id as u32));
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
