use log::info;

use crate::{
    classfile::class::{Class, ConstantPoolInfo, FieldInfo},
    common::{
        error::Throwable, object::Object, param::DataType, reference::Reference, stack_frame::StackFrame, value::{self, StackFrameValue, number_u64}
    },
    runtime::{
        heap::{self, Heap},
        metaspace::{self, Metaspace},
    },
};

pub fn putfield(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace)->Result<(),Throwable> {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let this_class = &metaspace.classes[frame.class];
    let field_ref: &ConstantPoolInfo = &this_class.constant_pool[index as usize];
    let value: StackFrameValue = frame.op_stack.pop().unwrap();
    let stack_frame_value: StackFrameValue = frame.op_stack.pop().unwrap();
    let object_id = {
        match stack_frame_value {
            StackFrameValue::Reference(id) => id,
            _ => panic!(),
        }
    };
    match field_ref {
        ConstantPoolInfo::Fieldref(_class_index, name_and_type_index) => {
            // let class_name = {
            //     let class_constant: &ConstantPoolInfo =
            //         &this_class.constant_pool[*_class_index as usize];
            //     match class_constant {
            //         ConstantPoolInfo::Class(name_index) => {
            //             let class_name_utf8: &ConstantPoolInfo =
            //                 &this_class.constant_pool[*name_index as usize];
            //             match class_name_utf8 {
            //                 ConstantPoolInfo::Utf8(class_name) => {
            //                     class_name.clone()
            //                 }
            //                 _ => panic!(),
            //             }
            //         }
            //         _ => panic!(),
            //     }
            // };
            let field_name = {
                let name_and_type: &ConstantPoolInfo =
                    &this_class.constant_pool[*name_and_type_index as usize];
                match name_and_type {
                    ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                        let field_name_utf8: &ConstantPoolInfo =
                            &this_class.constant_pool[*name_index as usize];
                        match field_name_utf8 {
                            ConstantPoolInfo::Utf8(field_name) => {
                                field_name.clone()
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            };
            
            let class_id =  heap.get_object_class_id(object_id as usize)?;
            let target_class = &metaspace.classes[class_id as usize];
            let (_field_index, offset, data_type) =
                metaspace.get_field_tupple(&(target_class.class_name.clone()), &field_name);

            match DataType::from(data_type) {
                DataType::Int => {
                    heap.put_field_i32(object_id, offset, value::as_i32(&value));
                }
                DataType::Long => {
                    heap.put_field_i64(object_id, offset, value::as_i64(&value));
                }
                DataType::Float => {
                    heap.put_field_f32(object_id, offset, value::as_f32(&value));
                }
                DataType::Byte => {
                    heap.put_field_i8(object_id, offset, value::as_i8(&value));
                }
                DataType::Char => {
                    heap.put_field_i16(object_id, offset, value::as_i16(&value));
                }
                DataType::Double => {
                    heap.put_field_f64(object_id, offset, value::as_f64(&value));
                }
                DataType::Reference(_) => {
                    heap.put_field_u32(object_id, offset, value::as_u32(&value));
                }
                DataType::Short => {
                    heap.put_field_i16(object_id, offset, value::as_i16(&value));
                }
                DataType::Boolean => {
                    heap.put_field_i8(object_id, offset, value::as_i8(&value));
                }
                DataType::Array {
                    element_type,
                    depth,
                } => {
                    if value != StackFrameValue::Null {
                        heap.put_field_u32(object_id, offset, value::as_u32(&value));
                    }
                }
                DataType::Unknown => todo!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 3;
    Ok(())
}

pub fn getfield(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) ->Result<(),Throwable>{
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let this_class = &mut metaspace.classes[frame.class];
    let field_ref: &ConstantPoolInfo = &this_class.constant_pool[index as usize];
    let stack_frame_value: StackFrameValue = frame.op_stack.pop().unwrap();
    let object_id = match stack_frame_value {
        StackFrameValue::Reference(id) => id,
        StackFrameValue::Null => {
            return Err(Throwable::Exception(crate::common::error::Exception::NullPointer("Null pointer exception".to_string())));
        }
        _=> {
            return Err(Throwable::Error(crate::common::error::JvmError::InternalError("Internal error".to_string())));
        }
    };
    let field_name = match field_ref {
        ConstantPoolInfo::Fieldref(class_index, name_and_type_index) => {
            let class_ref: &ConstantPoolInfo = &this_class.constant_pool[*class_index as usize];
            let name_and_type: &ConstantPoolInfo =
                &this_class.constant_pool[*name_and_type_index as usize];

            // let class_name = {
            //     match class_ref {
            //         ConstantPoolInfo::Class(class_name_index) => {
            //             let class_name_utf8: &ConstantPoolInfo =
            //                 &this_class.constant_pool[*class_name_index as usize];
            //             match class_name_utf8 {
            //                 ConstantPoolInfo::Utf8(class_name) => class_name,
            //                 _ => panic!(),
            //             }
            //         }
            //         _ => panic!(),
            //     }
            // };

            let field_name = {
                match name_and_type {
                    ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                        let field_name_utf8: &ConstantPoolInfo =
                            &this_class.constant_pool[*name_index as usize];
                        match field_name_utf8 {
                            ConstantPoolInfo::Utf8(field_name) => field_name,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            };

            field_name.clone()
        }
        _ => panic!(),
    };


      let class_id =  heap.get_object_class_id(object_id as usize)?;
      let target_class = &metaspace.classes[class_id as usize];
      let (_field_index, offset, data_type) = metaspace.get_field_tupple(&(target_class.class_name.clone()), &field_name);

    match data_type {
        DataType::Int => {
            let value = heap.get_field_i32(object_id, offset);
            frame.op_stack.push(StackFrameValue::Int(value));
        }
        DataType::Long => {
            let value = heap.get_field_i64(object_id, offset);
            frame.op_stack.push(StackFrameValue::Long(value));
        }
        DataType::Float => {
            let value = heap.get_field_f32(object_id, offset);
            frame.op_stack.push(StackFrameValue::Float(value));
        }
        DataType::Byte => {
            let value = heap.get_field_i8(object_id, offset);
            frame.op_stack.push(StackFrameValue::Byte(value));
        }
        DataType::Char => {
            let value = heap.get_field_i16(object_id, offset);
            frame.op_stack.push(StackFrameValue::Char(value));
        }
        DataType::Double => {
            let value = heap.get_field_f64(object_id, offset);
            frame.op_stack.push(StackFrameValue::Double(value));
        }
        DataType::Reference(_) => {
            let value = heap.get_field_ptr(object_id, offset);
            if value.is_none() {
                frame.op_stack.push(StackFrameValue::Null);

            }else{
                frame.op_stack.push(StackFrameValue::Reference(value.unwrap()));
            }
        }
        DataType::Short => {
            let value = heap.get_field_i16(object_id, offset);
            frame.op_stack.push(StackFrameValue::Short(value));
        }
        DataType::Boolean => {
            let value = heap.get_field_i8(object_id, offset);
            frame.op_stack.push(StackFrameValue::Boolean(value == 1));
        }
        DataType::Array {
            element_type,
            depth,
        } => {
            let value = heap.get_field_ptr(object_id, offset);
            if value.is_none() {
                frame.op_stack.push(StackFrameValue::Null);
            }else {
                frame.op_stack.push(StackFrameValue::Reference(value.unwrap()));
            }
        }
        DataType::Unknown => {
            panic!("Unknown data type")
        }
    }
    frame.pc += 3;
    Ok(())
}
