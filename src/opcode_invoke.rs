
use log::info;
use log::warn;

use crate::class::ConstantPoolInfo;
use crate::class::MethodInfo;
use crate::reference::reference::Reference;
use crate::runtime_data_area::get_class_name;
use crate::runtime_data_area::get_method_from_pool;
use crate::runtime_data_area::get_or_load_class;
use crate::runtime_data_area::get_reference;
use crate::runtime_data_area::VM_STACKS;
use crate::stack_frame::*;
use crate::u8c::u8s_to_u16;
use crate::value::value::StackFrameValue;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::f32::consts::E;
use crate::debug::*;
use crate::native::*;

pub fn get_method_for_invoke(frame: &StackFrame) -> Option<&MethodInfo> {
    let this_class = get_or_load_class(&frame.class_name).clone();
    // 使用 match 代替 if let 以减少嵌套，并处理 unwrap 导致的潜在 panic
    let (class_index, name_and_type_index) = match this_class
        .constant_pool
        .get(&u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]))
    {
        Some(ConstantPoolInfo::Methodref(class_index, name_and_type_index)) => {
            (class_index, name_and_type_index)
        }
        _ => return None,
    };

    // 通过链式调用减少嵌套
    let target_class_name = this_class
        .constant_pool
        .get(class_index)
        .and_then(|cp_info| match cp_info {
            ConstantPoolInfo::Class(name_index) => this_class.constant_pool.get(name_index),
            _ => None,
        })
        .and_then(|name_info| match name_info {
            ConstantPoolInfo::Utf8(target_class_name) => Some(target_class_name),
            _ => None,
        });

    let target_class = match target_class_name {
        Some(class_name_target) => get_or_load_class(class_name_target),
        None => return None,
    };

    // 继续减少嵌套并简化逻辑
    let (method_name, descriptor) = match this_class.constant_pool.get(name_and_type_index) {
        Some(ConstantPoolInfo::NameAndType(name_index, descriptor_index)) => {
            let method_name = match this_class.constant_pool.get(name_index) {
                Some(ConstantPoolInfo::Utf8(name)) => name,
                _ => return None,
            };
            let descriptor = match this_class.constant_pool.get(descriptor_index) {
                Some(ConstantPoolInfo::Utf8(desc)) => desc,
                _ => return None,
            };
            (method_name, descriptor)
        }
        _ => return None,
    };

    let mut method = get_method_from_pool(target_class.class_name.clone(), method_name.clone(), descriptor.clone());
    let mut curr_class = target_class;
    while method.is_none() {
       let super_class =  get_or_load_class(&curr_class.super_class_name);
       method = get_method_from_pool(super_class.class_name.clone(), method_name.clone(), descriptor.clone());
       curr_class = super_class;
    }
     method
}

pub fn invokespecial(frame: &mut StackFrame) {
    let clone_frame = &frame.clone();
    frame.pc += 3;
    let method = get_method_for_invoke(clone_frame).unwrap();
    //info!("{}--{}--{}",method.class_name,method.method_name,method.descriptor);
    //非native 方法
    let mut new_frame = init_stack_frame(frame, method, 1);
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

                    let mut method =
                        get_method_from_pool(class.class_name.clone(), method_name.clone(), descriptor.clone());
                        let mut curr_class = class;
                        while method.is_none() {
                           let super_class =  get_or_load_class(&curr_class.super_class_name);
                           method = get_method_from_pool(super_class.class_name.clone(), method_name.clone(), descriptor.clone());
                           curr_class = super_class;
                        }
                    let mut new_frame: StackFrame = init_stack_frame(frame, method.unwrap(), 1);
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
    let clone_frame = &frame.clone();
    let method = get_method_for_invoke(clone_frame).unwrap();
    //info!("{}--{}--{}",method.class_name,method.method_name,method.descriptor);
    //info!("{:?}", method.unwrap().method_name);
    let mut new_frame = init_stack_frame(frame, method, 1);
    //info!("{:?}",new_frame);
    let v = frame.op_stack.pop();
    match v {
        Some(obj) => {
            new_frame.local[0] = obj;
        }
        None => {
            panic!("error");
        }
    }
    frame.pc += 3;
    push_stack_frame(new_frame);
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
    let clone_frame = &frame.clone();
    let method = get_method_for_invoke(clone_frame).unwrap();
    frame.pc += 3;
    //我写的辅助调试输出的工具
    if method.method_name == "print20240503" {
       let v =  frame.op_stack.pop().unwrap();
       dprint(v);
    }else if method.access_flag & 0x0100 == 0 {
        let new_frame = init_stack_frame(frame, method, 0);
        push_stack_frame(new_frame);
    }else{
        run_static_native(method,frame);
    }
}
