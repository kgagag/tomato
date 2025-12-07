use log::info;

use crate::{classfile::class::ConstantPoolInfo, common::{stack_frame::StackFrame, value::StackFrameValue}, runtime::{heap::Heap, metaspace::Metaspace}, utils::{java, u8c::u8s_to_u16}};

extern crate env_logger;
extern crate log;



pub fn ldc(thread_id :u64, vm_stack: &mut Vec<StackFrame>,heap:&mut Heap,metaspace:&mut Metaspace) {
    //let  frame  = vm_stack.last_mut().unwrap();
    let pc = vm_stack.last().unwrap().pc;
    let index = vm_stack.last_mut().unwrap().code[pc + 1];
    let this_class: crate::classfile::class::Class = metaspace.get_class(&vm_stack.last_mut().unwrap().class_name).unwrap();
    let constant_pool_data = this_class.constant_pool.get(&(index as u16)).unwrap();
    //info!("{:?}",constant_pool_data);
    match constant_pool_data {
        ConstantPoolInfo::Float(f) => {
            vm_stack.last_mut().unwrap().op_stack.push(StackFrameValue::Float(*f));
        }
        ConstantPoolInfo::Integer(i) => {
            vm_stack.last_mut().unwrap().op_stack.push(StackFrameValue::Int(*i));
        }
        ConstantPoolInfo::Class(class) => {
            let constant_utf8_class = this_class.constant_pool.get(class).unwrap();
            match constant_utf8_class {
                ConstantPoolInfo::Utf8(class_name) => {
                    let class_obj = metaspace.get_class_constant(class_name);
                    if class_obj.is_none() {
                        let obj_id: u64 = java::create_class_object(thread_id,&class_name,heap, metaspace,vm_stack);
                        //put_into_class_constant_pool(class_name.clone(), obj_id);
                        metaspace.register_class_constant(class_name.to_string(), obj_id);
                        vm_stack.last_mut().unwrap().op_stack.push(StackFrameValue::Reference(obj_id));
                    }else {
                        vm_stack.last_mut().unwrap().op_stack.push(StackFrameValue::Reference(class_obj.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        ConstantPoolInfo::String(index) => {
            let utf8_constant = this_class.constant_pool.get(index).unwrap();
            match utf8_constant {
                ConstantPoolInfo::Utf8(str) => {
                    let str_obj = metaspace.get_string_constant(str);
                    if str_obj.is_some() {
                        vm_stack.last_mut().unwrap()
                            .op_stack
                            .push(StackFrameValue::Reference(str_obj.unwrap()))
                    } else {
                        let id =  java::create_string_object(str,thread_id,heap, metaspace,vm_stack);
                        vm_stack.last_mut().unwrap()
                            .op_stack
                            .push(StackFrameValue::Reference(id));
                    }
                }
                _ => panic!(),
            }
        }
        _=>panic!()
    }
    //frame.op_stack.push(frame.local.get(1).unwrap().clone());
    vm_stack.last_mut().unwrap().pc += 2;
}

pub fn ldc_w(vm_stack : &mut Vec<StackFrame>) {
    let frame: &mut StackFrame = vm_stack.last_mut().unwrap();
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn ldc2_w(vm_stack: &mut Vec<StackFrame>,metaspace:&mut Metaspace) {
    let frame: &mut StackFrame = vm_stack.last_mut().unwrap();
    let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
    let this_class = metaspace.get_class(&frame.class_name).unwrap();
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
