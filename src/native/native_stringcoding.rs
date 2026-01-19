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
                for (i, ch) in bytes.iter().enumerate() {
                   heap.put_basic_array_element(new_array_id as u32, i as usize, *ch as u64);
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
     Ok(())
    }


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
                for (i, ch) in chars.iter().enumerate() {
                   heap.put_basic_array_element(new_array_id as u32, i as usize, *ch as u64);
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