use log::info;

use crate::reference::reference::Reference;
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::class::*;
use crate::runtime_data_area::*;
pub fn putfield(frame: &mut StackFrame) {
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let field_ref: &ConstantPoolInfo = this_class.constant_pool.get(&(index)).unwrap();
    let value: StackFrameValue = frame.op_stack.pop().unwrap();
    let object: StackFrameValue = frame.op_stack.pop().unwrap();
    match field_ref {
        ConstantPoolInfo::Fieldref(class_index, name_and_type_index) => {
            let class_ref: &ConstantPoolInfo = this_class.constant_pool.get(&class_index).unwrap();
            let name_and_type: &ConstantPoolInfo =
                this_class.constant_pool.get(&name_and_type_index).unwrap();
            match class_ref {
                ConstantPoolInfo::Class(class_name_index) => {
                    let class_name_utf8: &ConstantPoolInfo =
                        this_class.constant_pool.get(&class_name_index).unwrap();
                    match class_name_utf8 {
                        ConstantPoolInfo::Utf8(class_name) => {
                            let target_class = get_or_load_class(&class_name);
                            match name_and_type {
                                ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                                    let field_name_utf8: &ConstantPoolInfo =
                                        this_class.constant_pool.get(&name_index).unwrap();
                                    match field_name_utf8 {
                                        ConstantPoolInfo::Utf8(field_name) => {
                                            for i in 0..target_class.field_info.len() {
                                                if target_class
                                                    .field_info
                                                    .get(i)
                                                    .unwrap()
                                                    .field_name
                                                    == *field_name
                                                {
                                                    match object {
                                                        StackFrameValue::Reference(
                                                            reference_id,
                                                        ) => {
                                                            let reference =
                                                                get_reference(&reference_id);
                                                            match reference {
                                                                Reference::Object(obj) => {
                                                                    obj.field
                                                                        .insert(i as u16, value);
                                                                    break;
                                                                }
                                                                _ => panic!(),
                                                            }
                                                        }
                                                        _ => panic!(),
                                                    }
                                                }
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 3;
}

pub fn getfield(frame: &mut StackFrame) {
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let field_ref: &ConstantPoolInfo = this_class.constant_pool.get(&(index)).unwrap();
    let object: StackFrameValue = frame.op_stack.pop().unwrap();
    match field_ref {
        ConstantPoolInfo::Fieldref(class_index, name_and_type_index) => {
            let class_ref: &ConstantPoolInfo = this_class.constant_pool.get(&class_index).unwrap();
            let name_and_type: &ConstantPoolInfo =
                this_class.constant_pool.get(&name_and_type_index).unwrap();
            match class_ref {
                ConstantPoolInfo::Class(class_name_index) => {
                    let class_name_utf8: &ConstantPoolInfo =
                        this_class.constant_pool.get(&class_name_index).unwrap();
                    match class_name_utf8 {
                        ConstantPoolInfo::Utf8(class_name) => {
                            let target_class = get_or_load_class(&class_name);
                            match name_and_type {
                                ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                                    let field_name_utf8: &ConstantPoolInfo =
                                        this_class.constant_pool.get(&name_index).unwrap();
                                    match field_name_utf8 {
                                        ConstantPoolInfo::Utf8(field_name) => {
                                            for i in 0..target_class.field_info.len() {
                                                if target_class
                                                    .field_info
                                                    .get(i)
                                                    .unwrap()
                                                    .field_name
                                                    .clone()
                                                    == field_name.clone()
                                                {
                                                    match object {
                                                        StackFrameValue::Reference(
                                                            reference_id,
                                                        ) => {
                                                            let reference =
                                                                get_reference(&reference_id);
                                                            match reference {
                                                                Reference::Object(obj) => {
                                                                    frame.op_stack.push(
                                                                        obj.field
                                                                            .get(&(i as u16))
                                                                            .unwrap()
                                                                            .clone(),
                                                                    );
                                                                    break;
                                                                }
                                                                _ => panic!(),
                                                            }
                                                        }
                                                        _ => panic!(),
                                                    }
                                                }
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 3;
}
