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

const ARRAY_TYPE_CHAR: u8 = 5;
const ARRAY_TYPE_BYTE: u8 = 8;

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
    
    // Pop unused parameters but validate they exist
    let _len = frame.popi64();
    let _off = frame.popi64();
    
    let sfv = frame.op_stack.pop()
        .ok_or_else(|| Throwable::Error(crate::common::error::JvmError::UnknownError("Stack underflow".to_string())))?;
    
    match sfv {
        StackFrameValue::Reference(reference_id) => {
            if !heap.is_array(reference_id as usize) {
                return Err(Throwable::Error(
                    crate::common::error::JvmError::UnknownError("Expected array reference".to_string()),
                ));
            }
            
            let len = heap.get_array_length(reference_id);
            let mut bytes: Vec<i8> = Vec::with_capacity(len as usize * 4); // Pre-allocate assuming max UTF-8 expansion
            
            for i in 0..len {
                let (atype, element) = heap.get_basic_array_element(reference_id, i as usize);
                
                let element_val = element.ok_or_else(|| 
                    Throwable::Error(crate::common::error::JvmError::UnknownError("Array element is None".to_string()))
                )? as u32;
                
                match atype {
                    ARRAY_TYPE_CHAR => {
                        let ch = char::from_u32(element_val)
                            .ok_or_else(|| Throwable::Error(crate::common::error::JvmError::UnknownError("Invalid character".to_string())))?;
                        
                        let char_bytes = u8c::char_to_bytes(ch);
                        for b in char_bytes {
                            bytes.push(b as i8);
                        }
                    },
                    ARRAY_TYPE_BYTE => {
                        bytes.push(element_val as i8);
                    },
                    _ => {
                        return Err(Throwable::Error(
                            crate::common::error::JvmError::UnknownError(
                                format!("Unsupported array type: {}", atype),
                            ),
                        ));
                    }
                }
            }
            
            // Create byte array
            let new_array_id = heap.create_basic_array(ARRAY_TYPE_BYTE, bytes.len() as u32, 1);
            
            for (i, &byte_val) in bytes.iter().enumerate() {
                heap.put_basic_array_element(new_array_id as u32, i, byte_val as u64);
            }
            
            frame.op_stack.push(StackFrameValue::Reference(new_array_id as u32));
        }
        _ => {
            return Err(Throwable::Error(
                crate::common::error::JvmError::UnknownError("Expected reference type".to_string()),
            ));
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
    
    // Pop unused parameters but validate they exist
    let _len = frame.popi64();
    let _off = frame.popi64();
    
    let sfv = frame.op_stack.pop()
        .ok_or_else(|| Throwable::Error(crate::common::error::JvmError::UnknownError("Stack underflow".to_string())))?;
    
    match sfv {
        StackFrameValue::Reference(reference_id) => {
            if !heap.is_array(reference_id as usize) {
                return Err(Throwable::Error(
                    crate::common::error::JvmError::UnknownError("Expected array reference".to_string()),
                ));
            }
            
            let len = heap.get_array_length(reference_id);
            let mut bytes: Vec<u8> = Vec::with_capacity(len as usize);
            
            for i in 0..len {
                let (atype, element) = heap.get_basic_array_element(reference_id, i as usize);
                
                if atype != ARRAY_TYPE_BYTE {
                    return Err(Throwable::Error(
                        crate::common::error::JvmError::UnknownError(
                            format!("Expected byte array, got type: {}", atype),
                        ),
                    ));
                }
                
                let element_val = element.ok_or_else(|| 
                    Throwable::Error(crate::common::error::JvmError::UnknownError("Array element is None".to_string()))
                )? as u8;
                
                bytes.push(element_val);
            }
            
            let chars = u8c::bytes_to_chars(bytes);
            
            // Create char array
            let new_array_id = heap.create_basic_array(ARRAY_TYPE_CHAR, chars.len() as u32, 1);
            
            for (i, &ch_val) in chars.iter().enumerate() {
                heap.put_basic_array_element(new_array_id as u32, i, ch_val as u64);
            }
            
            frame.op_stack.push(StackFrameValue::Reference(new_array_id as u32));
        }
        _ => {
            return Err(Throwable::Error(
                crate::common::error::JvmError::UnknownError("Expected reference type".to_string()),
            ));
        }
    }

    Ok(())
}