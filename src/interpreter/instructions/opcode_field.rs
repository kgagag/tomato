use log::info;

use crate::{
    classfile::class::{Class, ConstantPoolInfo, FieldInfo},
    common::{
        object::Object,
        param::DataType,
        reference::Reference,
        stack_frame::StackFrame,
        value::{self, number_u64, StackFrameValue},
    },
    runtime::{
        heap::{self, Heap},
        metaspace::{self, Metaspace},
        runtime_data_area::{get_class_name, get_or_load_class, get_reference},
    },
};

pub fn putfield(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let this_class = &metaspace.classes[frame.class];
    let field_ref: &ConstantPoolInfo = this_class.constant_pool.get(&(index)).unwrap();
    let value: StackFrameValue = frame.op_stack.pop().unwrap();
    info!("######{:?}#####", value);
    let stack_frame_value: StackFrameValue = frame.op_stack.pop().unwrap();
    let object_id = {
        match stack_frame_value {
            StackFrameValue::Reference(id) => id,
            _ => panic!(),
        }
    };
    match field_ref {
        ConstantPoolInfo::Fieldref(_class_index, name_and_type_index) => {
            let class_name = {
                let class_constant: &ConstantPoolInfo =
                    this_class.constant_pool.get(_class_index).unwrap();
                match class_constant {
                    ConstantPoolInfo::Class(name_index) => {
                        let class_name_utf8: &ConstantPoolInfo =
                            this_class.constant_pool.get(name_index).unwrap();
                        match class_name_utf8 {
                            ConstantPoolInfo::Utf8(class_name) => {
                                //需要找到对象id , 参数index , offset
                                // metaspace.get_field_tupple(&class_name, field_name)
                                //  object.field.insert(field_name.clone(), value);
                                class_name.clone()
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            };
            let field_name = {
                let name_and_type: &ConstantPoolInfo =
                    this_class.constant_pool.get(name_and_type_index).unwrap();
                match name_and_type {
                    ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                        let field_name_utf8: &ConstantPoolInfo =
                            this_class.constant_pool.get(name_index).unwrap();
                        match field_name_utf8 {
                            ConstantPoolInfo::Utf8(field_name) => {
                                //需要找到对象id , 参数index , offset
                                // metaspace.get_field_tupple(&class_name, field_name)
                                //  object.field.insert(field_name.clone(), value);
                                field_name.clone()
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            };

            let (_field_index, offset, data_type) =
                metaspace.get_field_tupple(&class_name, &field_name);
            match DataType::from(data_type) {
                DataType::Int => {
                    heap.put_field_i32(object_id, offset, &value::as_i32(&value));
                }
                DataType::Long => {
                    heap.put_field_i64(object_id, offset, &value::as_i64(&value));
                }
                DataType::Float => {
                    heap.put_field_f32(object_id, offset, &value::as_f32(&value));
                }
                DataType::Byte => {
                    heap.put_field_i8(object_id, offset, &value::as_i8(&value));
                }
                DataType::Char => {
                    heap.put_field_i16(object_id, offset, &value::as_i16(&value));
                }
                DataType::Double => {
                    heap.put_field_f64(object_id, offset, &value::as_f64(&value));
                }
                DataType::Reference(_) => {
                    heap.put_field_u32(object_id, offset, &value::as_u32(&value));
                }
                DataType::Short => {
                    heap.put_field_i16(object_id, offset, &value::as_i16(&value));
                }
                DataType::Boolean => {
                    heap.put_field_i8(object_id, offset, &value::as_i8(&value));
                }
                DataType::Array {
                    element_type,
                    depth,
                } => {
                    heap.put_field_u32(object_id, offset, &value::as_u32(&value));
                }
                DataType::Unknown => todo!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 3;
}

pub fn getfield(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let this_class = &mut metaspace.classes[frame.class];
    let field_ref: &ConstantPoolInfo = this_class.constant_pool.get(&(index)).unwrap();
    let stack_frame_value: StackFrameValue = frame.op_stack.pop().unwrap();
    let object_id = match stack_frame_value {
        StackFrameValue::Reference(id) => id,
        _ => panic!(),
    };
    let (class_name, field_name) = match field_ref {
        ConstantPoolInfo::Fieldref(class_index, name_and_type_index) => {
            let class_ref: &ConstantPoolInfo = this_class.constant_pool.get(class_index).unwrap();
            let name_and_type: &ConstantPoolInfo =
                this_class.constant_pool.get(name_and_type_index).unwrap();

            let class_name = {
                match class_ref {
                    ConstantPoolInfo::Class(class_name_index) => {
                        let class_name_utf8: &ConstantPoolInfo =
                            this_class.constant_pool.get(class_name_index).unwrap();
                        match class_name_utf8 {
                            ConstantPoolInfo::Utf8(class_name) => class_name,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            };

            let field_name = {
                match name_and_type {
                    ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                        let field_name_utf8: &ConstantPoolInfo =
                            this_class.constant_pool.get(name_index).unwrap();
                        match field_name_utf8 {
                            ConstantPoolInfo::Utf8(field_name) => field_name,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            };

            (class_name.clone(), field_name.clone())
        }
        _ => panic!(),
    };
    let (_field_index, offset, data_type) = metaspace.get_field_tupple(&class_name, &field_name);

    match data_type {
        DataType::Int => {
            let value = heap.get_field_i32(object_id, offset);
            frame.op_stack.push(StackFrameValue::Int(value));
            info!("========={:?}=============", value);
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
            frame.op_stack.push(StackFrameValue::Reference(value));
        }
        DataType::Short => {
            let value = heap.get_field_i16(object_id, offset);
            info!("========={:?}=============", value);
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
            frame.op_stack.push(StackFrameValue::Reference(value));
        }
        DataType::Unknown => {
            panic!("Unknown data type")
        }
    }
    frame.pc += 3;
}
