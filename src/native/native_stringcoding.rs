use std::{array, f64::consts::E};

use log::info;

use crate::{
    common::{
        error::Throwable,
        param::DataType,
        stack_frame::{self, StackFrame},
        value::StackFrameValue,
    },
    runtime::{heap::Heap, metaspace::Metaspace},
    utils::u8c,
};

/**
 * char数组转换成byte数组
 */
pub fn encode0(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let _len = frame.popi64();
    let _off = frame.popi64();
    let sfv = frame.op_stack.pop();
    match sfv {
        Some(sfv) => match sfv {
            StackFrameValue::Reference(reference_id) => {
                let flag = heap.is_array(reference_id as usize);
                if !flag {
                    return Err(Throwable::Error(
                        crate::common::error::JvmError::UnknownError("Unknown error".to_string()),
                    ));
                }
                let len = heap.get_array_length(reference_id);
                let mut bytes:Vec<i8> = Vec::new();
                
                for i in 0..len {
                    let (atype, element) = heap.get_basic_array_element(reference_id, i as usize);
                    if element.is_none() {
                        return Err(Throwable::Error(
                            crate::common::error::JvmError::UnknownError(
                                "Unknown error".to_string(),
                            ),
                        ));
                    }
                    let element = element.unwrap() as u32;
                    let ch = char::from_u32(element).unwrap();
                    if atype == 5{
                        u8c::char_to_bytes(ch).iter().for_each(|b| {
                            bytes.push(*b as i8);
                        });
                    }else if atype == 8 {
                            bytes.push(element as i8);
                    } else {
                        return Err(Throwable::Error(
                            crate::common::error::JvmError::UnknownError(
                                "Unknown error".to_string(),
                            ),
                        ));
                    }
                }
                
                //创建byte数组
               let new_array_id =  heap.create_basic_array(8, bytes.len() as u32, 1);
               for i in 0..bytes.len() {
                   heap.put_basic_array_element(new_array_id as u32, i as usize, bytes[i] as u64);
               }
               frame.op_stack.push(StackFrameValue::Reference(new_array_id as  u32));
            }
            _=> {
                return Err(Throwable::Error(
                    crate::common::error::JvmError::UnknownError("Unknown error".to_string()),
                ));
            }
        },
        _ => {
            return Err(Throwable::Error(
                crate::common::error::JvmError::UnknownError("Unknown error".to_string()),
            ))
        }
    }


    return Ok(());

    // let mut new_len = 0;
    // match reference {
    //     Reference::Array(array) => {
    //         //  info!("{:?}", array);
    //         let mut bytes: Vec<StackFrameValue> = Vec::new();
    //         for i in 0..array.data.len() {
    //             let sfv = array.data.get(i).unwrap();
    //             //info!("{:?}", sfv);
    //             match sfv {
    //                 StackFrameValue::CHARACTER(ch) => {
    //                     let items = u8c::char_to_bytes(*ch);
    //                     new_len += items.len();
    //                     for j in 0..items.len() {
    //                         bytes.push(StackFrameValue::Byte(*items.get(j).unwrap() as i8))
    //                     }
    //                 }
    //                 StackFrameValue::Byte(b) => {
    //                     bytes.push(StackFrameValue::Byte(*b as i8));
    //                 }
    //                 _ => panic!(),
    //             }
    //         }
    //         let new_array_id: u32 = runtime_data_area::create_array(
    //             new_len as u32,
    //             DataType::Array {
    //                 element_type: Box::new(DataType::Byte),
    //                 depth: (1),
    //             },
    //         );
    //         let reference = get_reference(&new_array_id).unwrap();
    //         //info!("{:?}", bytes);
    //         match reference {
    //             Reference::Array(array) => {
    //                 array.data = bytes;
    //             }
    //             _ => panic!(),
    //         }
    //         frame
    //             .op_stack
    //             .push(StackFrameValue::Reference(new_array_id))
    //     }
    //     _ => panic!(),
    }


/**
 * char数组转换成byte数组
 */
// pub fn decode0(frame: &mut StackFrame) {
//     let _len = frame.popi64();
//     let _off = frame.popi64();
//     let reference_id = frame.pop_reference();
//     let reference = get_reference(&reference_id).unwrap();
//     match reference {
//         Reference::Array(array) => {
//             // info!("{:?}", array);
//             let mut chars: Vec<StackFrameValue> = Vec::new();
//             let mut tmp: Vec<u8> = Vec::new();
//             for i in 0..array.data.len() {
//                 let sfv = array.data.get(i).unwrap();
//                 //info!("{:?}",sfv);
//                 match sfv {
//                     StackFrameValue::Byte(b) => {
//                         tmp.push(*b as u8);
//                     }
//                     StackFrameValue::Int(b) => {
//                         tmp.push(*b as u8);
//                     }
//                     _ => panic!(),
//                 }
//             }
//             let char_vec = u8c::bytes_to_chars(tmp);
//             for i in 0..char_vec.len() {
//                 chars.push(StackFrameValue::CHARACTER(*char_vec.get(i).unwrap()))
//             }
//             let new_array_id: u32 = runtime_data_area::create_array(
//                 char_vec.len() as u32,
//                 DataType::Array {
//                     element_type: Box::new(DataType::Char),
//                     depth: (1),
//                 },
//             );
//             let reference = get_reference(&new_array_id).unwrap();
//             match reference {
//                 Reference::Array(array) => {
//                     array.data = chars;
//                 }
//                 _ => panic!(),
//             }
//             frame
//                 .op_stack
//                 .push(StackFrameValue::Reference(new_array_id))
//         }
//         _ => panic!(),
//     }
// }


pub fn decode0(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let _len = frame.popi64();
    let _off = frame.popi64();
    let sfv = frame.op_stack.pop();
    match sfv {
        Some(sfv) => match sfv {
            StackFrameValue::Reference(reference_id) => {
                let flag = heap.is_array(reference_id as usize);
                if !flag {
                    return Err(Throwable::Error(
                        crate::common::error::JvmError::UnknownError("Unknown error".to_string()),
                    ));
                }
                let len = heap.get_array_length(reference_id);
                let mut bytes:Vec<u8> = Vec::new();
                for i in 0..len {
                    let (atype, element) = heap.get_basic_array_element(reference_id, i as usize);
                    if element.is_none() {
                        return Err(Throwable::Error(
                            crate::common::error::JvmError::UnknownError(
                                "Unknown error".to_string(),
                            ),
                        ));
                    }
                    let element = element.unwrap() as u8;
                    if atype == 8 {
                            bytes.push(element);
                    } else {
                        return Err(Throwable::Error(
                            crate::common::error::JvmError::UnknownError(
                                "Unknown error".to_string(),
                            ),
                        ));
                    }
                }
                let chars =  u8c::bytes_to_chars(bytes);
                //创建char数组
               let new_array_id =  heap.create_basic_array(5, chars.len() as u32, 1);
               for i in 0..chars.len() {
                   heap.put_basic_array_element(new_array_id as u32, i as usize, chars[i] as u64);
               }
               frame.op_stack.push(StackFrameValue::Reference(new_array_id as u32));
            }
            _=> {
                return Err(Throwable::Error(
                    crate::common::error::JvmError::UnknownError("Unknown error".to_string()),
                ));
            }
        },
        _ => {
            return Err(Throwable::Error(
                crate::common::error::JvmError::UnknownError("Unknown error".to_string()),
            ))
        }
    }

    Ok(())
}