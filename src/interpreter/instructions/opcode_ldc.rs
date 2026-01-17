use std::{f32::consts::E, string};

use log::info;

use crate::{
    classfile::class::ConstantPoolInfo,
    classloader::class_loader,
    common::{error::Throwable, stack_frame::StackFrame, value::StackFrameValue},
    runtime::{
        heap::{self, Heap},
        metaspace::Metaspace,
    },
    utils::{java, u8c::u8s_to_u16},
};

extern crate env_logger;
extern crate log;

pub fn ldc(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) ->Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (float_value, int_value, class_index,string_index) = {
        let frame = &mut vm_stack[frame_index];
        let index = frame.code[frame.pc + 1];
        let this_class = &mut metaspace.classes[frame.class];
        let constant_pool_data = &this_class.constant_pool[index as usize];
        match constant_pool_data {
            ConstantPoolInfo::Float(f) => (Some(f.clone()), None, None, None),
            ConstantPoolInfo::Integer(i) => (None, Some(i.clone()), None, None),
            ConstantPoolInfo::Class(class_index) => {
                let constant_utf8_class = &this_class.constant_pool[*class_index as usize];
                match constant_utf8_class {
                    ConstantPoolInfo::Utf8(class_name) => {
                        (None, None, Some(class_name.clone()), None)
                    }
                    _ => return Err(Throwable::Error(crate::common::error::JvmError::InternalError("Internal error".to_string())))
                }
            }
            ConstantPoolInfo::String(index) => {
                let utf8_constant = &this_class.constant_pool[*index as usize];
                match utf8_constant {
                    ConstantPoolInfo::Utf8(str) => (None, None, None, Some(str.clone())),
                 _ => return Err(Throwable::Error(crate::common::error::JvmError::InternalError("Internal error".to_string())))
                }
            }
             _ => return Err(Throwable::Error(crate::common::error::JvmError::InternalError("Internal error".to_string())))
        }
    };

    if let Some(float_value) = float_value {
        vm_stack[frame_index]
            .op_stack
            .push(StackFrameValue::Float(float_value));
    } else if let Some(int_value) = int_value {
        vm_stack[frame_index]
            .op_stack
            .push(StackFrameValue::Int(int_value));
    } else if let Some(string) = string_index {
        //let string_obj = heap::create_string_object(string_index, vm_stack, heap, metaspace)?;
        let string_obj =heap.get_constant_string_pool(&string);
        if string_obj.is_some() {
             vm_stack[frame_index]
                .op_stack
                .push(StackFrameValue::Reference(*string_obj.unwrap()));
        }else{
            let string_obj = java::create_string_object(string,vm_stack,heap,metaspace)?;
            vm_stack[frame_index]
                .op_stack
                .push(StackFrameValue::Reference(string_obj));
        }
    } else if let Some(class_name) = class_index {
        //确保这个类已被加载
        //let class_id: usize = (class_loader::find_class(&class_name, vm_stack, heap, metaspace)?).id;
        let class_obj = heap.get_constant_pool_class(&class_name);
        if class_obj.is_none() {
            let obj_id: u32 = java::create_class_object(&class_name,vm_stack,heap,metaspace)?;
            heap.put_into_class_constant_pool(class_name, obj_id);
            vm_stack[frame_index].op_stack.push(StackFrameValue::Reference(obj_id));
        } else {
             vm_stack[frame_index]
                .op_stack
                .push(StackFrameValue::Reference(*class_obj.unwrap()));
        }
    }
     vm_stack[frame_index].pc += 2;
     Ok(())
}

pub fn ldc_w(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn ldc2_w(vm_stack: &mut Vec<StackFrame>,heap:&mut Heap,metaspace: &mut Metaspace) ->Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let index = u8s_to_u16(&vm_stack[frame_index].code[vm_stack[frame_index].pc + 1..vm_stack[frame_index].pc + 3]);
    let class_name =vm_stack[frame_index].class_name.clone();
    let this_class = class_loader::find_class(&class_name,vm_stack,heap,metaspace)?;
    let constant_pool_data = &this_class.constant_pool[index as usize];
    match constant_pool_data {
        ConstantPoolInfo::Long(l) => {
            vm_stack[frame_index].op_stack.push(StackFrameValue::Long(*l));
        }
        ConstantPoolInfo::Double(d) => {
            vm_stack[frame_index].op_stack.push(StackFrameValue::Double(*d));
        }
        _ => panic!(),
    }
    vm_stack[frame_index].pc += 3;
    Ok(())
}
