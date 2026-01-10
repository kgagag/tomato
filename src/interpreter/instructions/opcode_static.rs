use clap::FromArgMatches;
use log::info;

use crate::{
    classfile::class::ConstantPoolInfo,
    classloader::class_loader,
    common::{error::Throwable, stack_frame::StackFrame},
    runtime::{
        heap::Heap,
        metaspace::Metaspace
    },
};

pub fn putstatic(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) ->Result<(),Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (class_name, field_name) = {
        let frame = &mut vm_stack[frame_index];
        let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
        frame.pc += 3;
        //let class_name = get_class_name(&frame.class);
        let this_class = &mut metaspace.classes[frame.class];
        let field_ref = &this_class
            .constant_pool
            [index as usize];
        if let ConstantPoolInfo::Fieldref(class_index, name_and_type_index) = field_ref {
            let class_name_utf8 = match &this_class.constant_pool[*class_index as usize] {
                ConstantPoolInfo::Class(class_name_index) => {
                    &this_class.constant_pool[*class_name_index as usize]
                }
                _ => panic!(),
            };
           

            let (class_name, field_name) = {
                if let ConstantPoolInfo::Utf8(class_name) = class_name_utf8 {
                    let name_and_type: &ConstantPoolInfo = &this_class
                        .constant_pool[*name_and_type_index as usize];
                    // let mut target_class = class_loader::find_class(class_name,vm_stack,heap,metaspace);
                    if let ConstantPoolInfo::NameAndType(name_index, _) = name_and_type {
                        let field_name_utf8 = &this_class
                            .constant_pool[*name_index as usize];
                        if let ConstantPoolInfo::Utf8(field_name) = field_name_utf8 {
                            (class_name, field_name)
                        } else {
                            panic!("Unexpected constant pool info type");
                        }
                    } else {
                        panic!("Unexpected name and type");
                    }
                } else {
                    panic!("Unexpected class name UTF-8");
                }
            };
            (class_name.clone(), field_name.clone())
        } else {
            panic!("Unexpected field reference");
        }
    };
    //找到这个class,，然后给他的static 字段赋值
    let target_class = class_loader::find_class(&class_name, vm_stack, heap, metaspace)?;
    let field = target_class.field_info.get_mut(&field_name).unwrap();
    field.value = vm_stack[frame_index].op_stack.pop().unwrap();
    Ok(())
}

pub fn getstatic(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) ->Result<(),Throwable>{
    let frame_index = vm_stack.len() - 1;
    let (class_name, field_name) = {
        let frame = &mut vm_stack[frame_index];
        let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
        frame.pc += 3;
        let this_class = &metaspace.classes[frame.class];
        let field_ref = &this_class
            .constant_pool[index as usize];
        if let ConstantPoolInfo::Fieldref(class_index, name_and_type_index) = field_ref {
            let class_name_utf8 = match &this_class.constant_pool[*class_index as usize] {
                ConstantPoolInfo::Class(class_name_index)=> {
                    &this_class.constant_pool[*class_name_index as usize]
                }
                _ => panic!(),
            };
            if let ConstantPoolInfo::Utf8(class_name) = class_name_utf8 {
                let name_and_type = &this_class
                    .constant_pool[*name_and_type_index as usize];
                if let ConstantPoolInfo::NameAndType(name_index, _) = name_and_type {
                    let field_name_utf8 = &this_class
                        .constant_pool[*name_index as usize];
                    if let ConstantPoolInfo::Utf8(field_name) = field_name_utf8 {
                        (class_name.clone(), field_name.clone())
                    } else {
                        panic!("Unexpected constant pool info type");
                    }
                } else {
                    panic!("Unexpected name and type");
                }
            } else {
                panic!("Unexpected class name UTF-8");
            }
        } else {
            panic!("Unexpected field reference");
        }
    };
    let target_class = class_loader::find_class(&class_name, vm_stack, heap, metaspace)?;
    let field_info = target_class.field_info.get_mut(&field_name).unwrap();
    vm_stack[frame_index].op_stack.push(field_info.value);
    Ok(())
}
