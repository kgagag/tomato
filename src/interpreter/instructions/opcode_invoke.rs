use log::info;
use log::warn;
use tiny_http::Method;

use std::cell::UnsafeCell;
use std::collections::HashMap;

use crate::classfile::class;
use crate::classfile::class::ConstantPoolInfo;
use crate::classloader::class_loader;
use crate::common::param::DataType;
use crate::common::reference::Reference;
use crate::common::stack_frame::create_stack_frame;
use crate::common::stack_frame::init_stack_frame;
use crate::common::stack_frame::push_stack_frame;
use crate::common::stack_frame::StackFrame;
use crate::common::value::number_to_u32tuple;
use crate::common::value::StackFrameValue;
use crate::native::native::run_native;
use crate::runtime::heap::Heap;
use crate::runtime::metaspace;
use crate::runtime::metaspace::Metaspace;
use crate::runtime::runtime_data_area::get_class_name;
use crate::runtime::runtime_data_area::get_method_from_pool;
use crate::runtime::runtime_data_area::get_or_load_class;
use crate::runtime::runtime_data_area::get_reference;
use crate::runtime::runtime_data_area::VM_STACKS;
use crate::utils::debug::dprint;
use crate::utils::u8c::u8s_to_u16;

pub fn invokespecial(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let (target_class_name, method_name, descriptor) = {
        let this_class = &metaspace.classes[vm_stack[frame_index].class];
        let (class_index, name_and_type_index) = match this_class.constant_pool.get(&u8s_to_u16(
            &vm_stack[frame_index].code
                [(vm_stack[frame_index].pc + 1)..(vm_stack[frame_index].pc + 3)],
        )) {
            Some(ConstantPoolInfo::Methodref(class_index, name_and_type_index)) => {
                (class_index, name_and_type_index)
            }
            _ => panic!(),
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

        // 继续减少嵌套并简化逻辑
        let (method_name, descriptor) = match this_class.constant_pool.get(name_and_type_index) {
            Some(ConstantPoolInfo::NameAndType(name_index, descriptor_index)) => {
                let method_name = match this_class.constant_pool.get(name_index) {
                    Some(ConstantPoolInfo::Utf8(name)) => name,
                    _ => panic!(),
                };
                let descriptor = match this_class.constant_pool.get(descriptor_index) {
                    Some(ConstantPoolInfo::Utf8(desc)) => desc,
                    _ => panic!(),
                };
                (method_name, descriptor)
            }
            _ => panic!(),
        };
        (
            target_class_name.unwrap().clone(),
            method_name.clone(),
            descriptor.clone(),
        )
    };
    //let target_class: &mut class::Class = class_loader::find_class(&target_class_name, vm_stack, heap, metaspace);
    let (method,class) = metaspace.get_method_from_root(&target_class_name, &method_name, &descriptor);
    //先移动
    vm_stack[frame_index].pc += 3;
    let mut method = method.unwrap();
    if method.access_flag & 0x0100 == 0 {
        let mut new_frame = init_stack_frame(&mut vm_stack[frame_index], &method,class, 1);
        let v = vm_stack[frame_index].op_stack.pop();
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
        run_native(&mut method, &mut vm_stack[frame_index]);
    }
}

/*
pub fn invokeinterface(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    //let class_name =metaspace.classes[];
    let this_class = &mut metaspace.classes[vm_stack[frame_index].class];
    let cnt = vm_stack[frame_index].code[vm_stack[frame_index].pc + 3];
    let mut tmp: Vec<StackFrameValue> = Vec::new();
    for _i in 1..cnt {
        tmp.push(vm_stack[frame_index].op_stack.pop().unwrap());
    }
    let v = vm_stack[frame_index].op_stack.pop().unwrap();
    for _i in 1..cnt {
        vm_stack[frame_index].op_stack.push(tmp.pop().unwrap());
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
                        .get(&u8s_to_u16(&vm_stack[frame_index].code[(vm_stack[frame_index].pc + 1)..(vm_stack[frame_index].pc + 3)]))
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

                    let mut method = metaspace.get_method_from_pool(
                        &class.class_name,
                        &method_name,
                        &descriptor,
                    );
                    let mut curr_class = class;
                    while method.is_none() {
                        let super_class = get_or_load_class(&curr_class.super_class_name);
                        method = metaspace.get_method_from_pool(
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
*/
