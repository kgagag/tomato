use log::info;

use crate::{classfile::class::ConstantPoolInfo, common::{stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{ get_class_name, get_constant_pool_class, get_constant_pool_str, get_or_load_class, put_into_class_constant_pool}, utils::{java, u8c::u8s_to_u16}};

extern crate env_logger;
extern crate log;



pub fn ldc(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1];
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let constant_pool_data = this_class.constant_pool.get(&(index as u16)).unwrap();
    //info!("{:?}",constant_pool_data);
    match constant_pool_data {
        ConstantPoolInfo::Float(f) => {
            frame.op_stack.push(StackFrameValue::Float(*f));
        }
        ConstantPoolInfo::Integer(i) => {
            frame.op_stack.push(StackFrameValue::Int(*i));
        }
        ConstantPoolInfo::Class(class) => {
            let constant_utf8_class = this_class.constant_pool.get(class).unwrap();
            match constant_utf8_class {
                ConstantPoolInfo::Utf8(class_name) => {
                    let class_obj = get_constant_pool_class(class_name);
                    if class_obj.is_none() {
                        let obj_id: u64 =   java::create_class_object(class_name);
                        put_into_class_constant_pool(class_name.clone(), obj_id);
                        frame.op_stack.push(StackFrameValue::Reference(obj_id));
                    }else {
                        frame.op_stack.push(StackFrameValue::Reference(*class_obj.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        ConstantPoolInfo::String(index) => {
            let utf8_constant = this_class.constant_pool.get(index).unwrap();
            match utf8_constant {
                ConstantPoolInfo::Utf8(str) => {
                    let str_obj = get_constant_pool_str(str);
                    if str_obj.is_some() {
                        frame
                            .op_stack
                            .push(StackFrameValue::Reference(str_obj.unwrap()))
                    } else {
                        frame
                            .op_stack
                            .push(StackFrameValue::Reference(java::create_string_object(str.clone())));
                    }
                }
                _ => panic!(),
            }
        }
        _=>panic!()
    }
    //frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 2;
}

pub fn ldc_w(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn ldc2_w(frame: &mut StackFrame) {
    let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let constant_pool_data = this_class.constant_pool.get(&index).unwrap();
    match constant_pool_data {
        ConstantPoolInfo::Long(l) => {
            frame.op_stack.push(StackFrameValue::Long(*l));
        }
        ConstantPoolInfo::Double(d) => {
            frame.op_stack.push(StackFrameValue::Double(*d));
        }
        _ => panic!(),
    }
    frame.pc += 3;
}
