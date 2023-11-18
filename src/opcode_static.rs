use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
use env_logger;
use log::{error, info, warn};
use std::env;
use crate::u8c::*;
use crate::runtime_data_area::*;
use crate::class::*;

pub fn putstatic(frame: &mut StackFrame) {
    let index = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);

    if let ConstantPoolInfo::Fieldref(class_index, name_and_type_index) = &this_class.constant_pool[&index] {
        if let ConstantPoolInfo::Class(class_name_index) = &this_class.constant_pool[class_index] {
            if let ConstantPoolInfo::Utf8(class_name) = &this_class.constant_pool[class_name_index] {
                let target_class = get_or_load_class(class_name);
                if let ConstantPoolInfo::NameAndType(name_index, descriptor_index) = &this_class.constant_pool[name_and_type_index] {
                    if let ConstantPoolInfo::Utf8(field_name) = &this_class.constant_pool[name_index] {
                        for field in &mut target_class.field_info {
                            if &field.field_name == field_name {
                                field.value = frame.op_stack.pop().unwrap();
                            }
                        }
                    } else {
                        panic!("Unexpected constant pool entry type for field name");
                    }
                } else {
                    panic!("Unexpected constant pool entry type for name and type");
                }
            } else {
                panic!("Unexpected constant pool entry type for class name");
            }
        } else {
            panic!("Unexpected constant pool entry type for class reference");
        }
    } else {
        panic!("Unexpected constant pool entry type for field reference");
    }

    frame.pc += 3;
}


pub fn getstatic(frame: &mut StackFrame) {
    let index = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    if let ConstantPoolInfo::Fieldref(class_index, name_and_type_index) = &this_class.constant_pool[&index] {
        if let ConstantPoolInfo::Class(class_name_index) = &this_class.constant_pool[class_index] {
            if let ConstantPoolInfo::Utf8(class_name) = &this_class.constant_pool[class_name_index] {
                let target_class = get_or_load_class(class_name);
                if let ConstantPoolInfo::NameAndType(name_index, _) = &this_class.constant_pool[name_and_type_index] {
                    if let ConstantPoolInfo::Utf8(field_name) = &this_class.constant_pool[name_index] {
                        if let Some(field) = target_class.field_info.iter().find(|f| &f.field_name == field_name) {
                            frame.op_stack.push(field.value.clone());
                        } else {
                            panic!("Field not found: {}", field_name);
                        }
                    } else {
                        panic!("Unexpected constant pool entry type for field name");
                    }
                } else {
                    panic!("Unexpected constant pool entry type for name and type");
                }
            } else {
                panic!("Unexpected constant pool entry type for class name");
            }
        } else {
            panic!("Unexpected constant pool entry type for class reference");
        }
    } else {
        panic!("Unexpected constant pool entry type for field reference");
    }
    frame.pc += 3;
}
