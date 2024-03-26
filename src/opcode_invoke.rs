use log::info;

use crate::class::ConstantPoolInfo;
use crate::class::MethodInfo;
use crate::runtime_data_area::get_class_name;
use crate::runtime_data_area::get_method_from_pool;
use crate::runtime_data_area::get_or_load_class;
use crate::runtime_data_area::VM_STACKS;
use crate::stack_frame::init_stack_frame;
use crate::stack_frame::StackFrame;
use crate::u8c::u8s_to_u16;
use std::cell::UnsafeCell;
use std::collections::HashMap;

pub fn get_method_for_invoke(frame: &StackFrame) -> Option<&MethodInfo> {
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    if let ConstantPoolInfo::Methodref(class_index, name_and_type_index) = &this_class
        .constant_pool
        .get(&u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]))
        .unwrap()
    {
        if let ConstantPoolInfo::Class(name_index) =
            &this_class.constant_pool.get(&class_index).unwrap()
        {
            if let ConstantPoolInfo::Utf8(class_name) =
                &this_class.constant_pool.get(&name_index).unwrap()
            {
                let target_class = get_or_load_class(&class_name);
                if let ConstantPoolInfo::NameAndType(name_index, descriptor_index) =
                    &this_class.constant_pool.get(&name_and_type_index).unwrap()
                {
                    if let ConstantPoolInfo::Utf8(method_name) =
                        &this_class.constant_pool.get(&name_index).unwrap()
                    {
                        if let ConstantPoolInfo::Utf8(descriptor) =
                            &this_class.constant_pool.get(&descriptor_index).unwrap()
                        {
                            return Some(get_method_from_pool(
                                target_class.class_name.clone(),
                                method_name.clone(),
                                descriptor.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }
    None
}

//对象的初始化方法
pub fn invokespecial(frame: &mut StackFrame) {
    let clone_frame = &frame.clone();
    let method = get_method_for_invoke(&clone_frame);
    //非native 方法
    let mut new_frame = init_stack_frame(frame, method.unwrap(),1);
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
    frame.pc += 3;
}

pub fn invokevirtual(frame: &mut StackFrame) {
    let clone_frame = &frame.clone();
    let method = get_method_for_invoke(&clone_frame);
    //info!("{:?}",method);
    let mut new_frame = init_stack_frame(frame, method.unwrap(),1);
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
    frame.pc += 3;
}

pub fn push_stack_frame(mut stack_frame: StackFrame) {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
        VM_STACKS.lock().unwrap();
    unsafe {
        let map: &mut HashMap<u32, Vec<StackFrame>> = &mut *data.get();
        if stack_frame.vm_stack_id == 0 {
            for i in 1..0xFFFFFFFF as u32 {
                if !map.contains_key(&i) {
                    stack_frame.vm_stack_id = i;
                    let mut stack_frames: Vec<StackFrame> = Vec::new();
                    stack_frames.push(stack_frame);
                    map.insert(i, stack_frames);
                    break;
                }
            }
        } else {
            map.get_mut(&stack_frame.vm_stack_id)
                .unwrap()
                .push(stack_frame);
        }
    }
    drop(data);
}


pub fn invokestatic(frame: &mut StackFrame) {
    let clone_frame = &frame.clone();
    let method: Option<&MethodInfo> = get_method_for_invoke(&clone_frame);
    let mut new_frame = init_stack_frame(frame, method.unwrap(),0);
    let v = frame.op_stack.pop();
    push_stack_frame(new_frame);
    frame.pc += 3;
}