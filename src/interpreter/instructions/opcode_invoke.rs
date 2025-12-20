use log::info;
use log::warn;


use std::cell::UnsafeCell;
use std::collections::HashMap;

use crate::classfile::class::ConstantPoolInfo;
use crate::common::reference::Reference;
use crate::common::stack_frame::init_stack_frame;
use crate::common::stack_frame::push_stack_frame;
use crate::common::stack_frame::StackFrame;
use crate::common::value::StackFrameValue;
use crate::native::native::run_native;
use crate::runtime::runtime_data_area::get_class_name;
use crate::runtime::runtime_data_area::get_method_from_pool;
use crate::runtime::runtime_data_area::get_or_load_class;
use crate::runtime::runtime_data_area::get_reference;
use crate::runtime::runtime_data_area::VM_STACKS;
use crate::utils::debug::dprint;
use crate::utils::u8c::u8s_to_u16;



pub fn invokespecial(frame: &mut StackFrame) {
    let method = frame.get_method_for_invoke().unwrap();
    frame.pc += 3;
    if method.access_flag & 0x0100 == 0 {
        let mut new_frame = init_stack_frame(frame, &method, 1);
        let v = frame.op_stack.pop();
        match v {
            Some(obj) => {
                new_frame.local[0] = obj;
            }
            None => {
                panic!("error");
            }
        }
        push_stack_frame(new_frame);
    } else {
        run_native(&method, frame);
    }
}

pub fn invokeinterface(frame: &mut StackFrame) {
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name).clone();
    let cnt = frame.code[frame.pc + 3];
    let mut tmp: Vec<StackFrameValue> = Vec::new();
    for _i in 1..cnt {
        tmp.push(frame.op_stack.pop().unwrap());
    }
    let v = frame.op_stack.pop().unwrap();
    for _i in 1..cnt {
        frame.op_stack.push(tmp.pop().unwrap());
    }
    match v {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Object(object) => {
                    let class_name = get_class_name(&object.class);
                    let class = get_or_load_class(&class_name);
                    let (_class_index, name_and_type_index) = match this_class
                        .constant_pool
                        .get(&u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]))
                    {
                        Some(ConstantPoolInfo::InterfaceMethodref(
                            class_index,
                            name_and_type_index,
                        )) => (class_index, name_and_type_index),
                        _ => panic!(),
                    };

                    let (method_name, descriptor) =
                        match this_class.constant_pool.get(name_and_type_index) {
                            Some(ConstantPoolInfo::NameAndType(name_index, descriptor_index)) => {
                                let method_name = match this_class.constant_pool.get(name_index) {
                                    Some(ConstantPoolInfo::Utf8(name)) => name,
                                    _ => panic!(),
                                };
                                let descriptor =
                                    match this_class.constant_pool.get(descriptor_index) {
                                        Some(ConstantPoolInfo::Utf8(desc)) => desc,
                                        _ => panic!(),
                                    };
                                (method_name, descriptor)
                            }
                            _ => panic!(),
                        };

                    let mut method = get_method_from_pool(
                        &class.class_name,
                        &method_name,
                        &descriptor,
                    );
                    let mut curr_class = class;
                    while method.is_none() {
                        let super_class = get_or_load_class(&curr_class.super_class_name);
                        method = get_method_from_pool(
                            &super_class.class_name,
                            &method_name,
                            &descriptor,
                        );
                        curr_class = super_class;
                    }
                    let mut new_frame: StackFrame = init_stack_frame(frame, &method.unwrap(), 1);
                    new_frame.local[0] = v;
                    push_stack_frame(new_frame);
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 4;
}

pub fn invokevirtual(frame: &mut StackFrame) {
    let method = frame.get_method_for_invoke().unwrap();
    frame.pc += 3;
    let param_len = method.param.len();
    let sfv = frame
        .op_stack
        .get(frame.op_stack.len() - param_len - 1)
        .unwrap();
    let target_method = match sfv {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(id).unwrap();
            match reference {
                Reference::Object(object) => {
                    let class_name = get_class_name(&object.class);
                    let mut curr_class_name = class_name.clone();
                    let mut target_method = get_method_from_pool(
                        &curr_class_name,
                        &method.method_name,
                        &method.descriptor,
                    );
                    while target_method.is_none() {
                        let clazz = get_or_load_class(&(curr_class_name.clone()));
                        curr_class_name = clazz.super_class_name.clone();
                        target_method = get_method_from_pool(
                            &curr_class_name,
                            &method.method_name,
                            &method.descriptor,
                        )
                    }
                    target_method
                }
                Reference::Array(_array) =>{
                    Some(method)
                }
            }
        }
        _ => panic!(),
    };

    let m= target_method.unwrap();
    if m.access_flag & 0x0100 == 0 {
        let mut new_frame = init_stack_frame(frame, &m, 1);
        let v = frame.op_stack.pop();
        match v {
            Some(obj) => {
                new_frame.local[0] = obj;
            }
            None => {
                panic!("error");
            }
        }
        push_stack_frame(new_frame);
    } else {
        run_native(&m, frame);
    }
}

pub fn get_frames(vm_stack_id: &u32) -> &Vec<StackFrame> {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
        VM_STACKS.lock().unwrap();
    unsafe {
        let map: &mut HashMap<u32, Vec<StackFrame>> = &mut *data.get();
        return map.get(vm_stack_id).unwrap();
    }
}

pub fn invokestatic(frame: &mut StackFrame) {
    let method = frame.get_method_for_invoke().unwrap();
    frame.pc += 3;
    //我写的辅助调试输出的工具
    if method.method_name == "print20240503" {
        let v = frame.op_stack.pop().unwrap();
        dprint(v);
    } else if method.access_flag & 0x0100 == 0 {
        let new_frame = init_stack_frame(frame, &method, 0);
        push_stack_frame(new_frame);
    } else {
        run_native(&method, frame);
    }
}
