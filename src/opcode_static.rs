use crate::stack_frame::StackFrame;
extern crate env_logger;
extern crate log;
use crate::class::*;
use crate::runtime_data_area::*;
use log::{error, info, warn};
pub fn putstatic(frame: &mut StackFrame) {
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let field_ref: &ConstantPoolInfo = this_class.constant_pool.get(&(index)).unwrap();
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
                                ConstantPoolInfo::NameAndType(name_index, descritor_index) => {
                                    let field_name_utf8: &ConstantPoolInfo =
                                        this_class.constant_pool.get(&name_index).unwrap();
                                    match field_name_utf8 {
                                        ConstantPoolInfo::Utf8(field_name) => {
                                            for i in 0..target_class.field_info.len() {
                                                let field: &mut FieldInfo =
                                                    target_class.field_info.get_mut(i).unwrap();
                                                if field
                                                    .field_name
                                                    .clone()
                                                    == field_name.clone()
                                                {
                                                    field.value =  frame.op_stack.pop().unwrap();
                                                    break;
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

pub fn getstatic(frame: &mut StackFrame) {
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let field_ref: &ConstantPoolInfo = this_class.constant_pool.get(&(index)).unwrap();
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
                                ConstantPoolInfo::NameAndType(name_index, descritor_index) => {
                                    let field_name_utf8: &ConstantPoolInfo =
                                        this_class.constant_pool.get(&name_index).unwrap();
                                    match field_name_utf8 {
                                        ConstantPoolInfo::Utf8(field_name) => {
                                            for i in 0..target_class.field_info.len() {
                                                let field = target_class.field_info.get(i).unwrap();
                                                if target_class
                                                    .field_info
                                                    .get(i)
                                                    .unwrap()
                                                    .field_name
                                                    .clone()
                                                    == field_name.clone()
                                                {
                                                    error!("{:?}",field);
                                                    frame.op_stack.push(field.value.clone());
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
