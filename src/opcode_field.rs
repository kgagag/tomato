
use std::f32::consts::E;

use clap::Id;
use log::info;

use crate::object;
use crate::object::Object;
use crate::param::param::DataType;
use crate::reference::reference::Reference;
use crate::stack_frame;
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
    let stack_frame_value: StackFrameValue = frame.op_stack.pop().unwrap();
    let  object ;
    match stack_frame_value {
        StackFrameValue::Reference(id) =>{
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Object(obj) =>{
                    object = obj;
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }

    match field_ref {
        ConstantPoolInfo::Fieldref(class_index, name_and_type_index) => {
            let name_and_type: &ConstantPoolInfo =
                this_class.constant_pool.get(name_and_type_index).unwrap();
                match name_and_type {
                    ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                        let field_name_utf8: &ConstantPoolInfo =
                            this_class.constant_pool.get(name_index).unwrap();
                        match field_name_utf8 {
                            ConstantPoolInfo::Utf8(field_name) => {
                                object.field.insert(field_name.clone(), value);
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
    let stack_frame_value: StackFrameValue = frame.op_stack.pop().unwrap();
    let mut object:&Object;
    match stack_frame_value {
        StackFrameValue::Reference(id) =>{
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Object(obj) =>{
                    object = obj;
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }

    match field_ref {
        ConstantPoolInfo::Fieldref(class_index, name_and_type_index) => {
            let class_ref: &ConstantPoolInfo = this_class.constant_pool.get(class_index).unwrap();
            let name_and_type: &ConstantPoolInfo =
                this_class.constant_pool.get(name_and_type_index).unwrap();
            match class_ref {
                ConstantPoolInfo::Class(class_name_index) => {
                    let class_name_utf8: &ConstantPoolInfo =
                        this_class.constant_pool.get(class_name_index).unwrap();
                    match class_name_utf8 {
                        ConstantPoolInfo::Utf8(class_name) => {
                            let mut target_class: &mut Class = get_or_load_class(class_name);
                            match name_and_type {
                                ConstantPoolInfo::NameAndType(name_index, _descritor_index) => {
                                    let field_name_utf8: &ConstantPoolInfo =
                                        this_class.constant_pool.get(name_index).unwrap();
                                    match field_name_utf8 {
                                        ConstantPoolInfo::Utf8(field_name) => {
                                            while target_class.field_info.get(field_name).is_none() {
                                                target_class = get_or_load_class(&target_class.super_class_name);
                                            }
                                            let op = object.field.get(field_name);
                                            if op.is_none() {
                                                let field_info: &FieldInfo = target_class.field_info.get(field_name).unwrap();
                                                if field_info.data_type == DataType::Char || field_info.data_type == DataType::Short
                                                || field_info.data_type == DataType::Int
                                                || field_info.data_type == DataType::Long
                                                || field_info.data_type == DataType::Byte
                                                || field_info.data_type == DataType::Float
                                                || field_info.data_type == DataType::Double {
                                                    frame.op_stack.push(StackFrameValue::Short(0));
                                                }else {
                                                    frame.op_stack.push(StackFrameValue::Null);
                                                }

                                            }else{
                                             //   info!("{}--{}--{:?}",class_name,field_name,op);
                                                frame.op_stack.push(op.unwrap().clone());
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
