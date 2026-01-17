use std::fmt::format;
use std::u32;

use crate::classfile::class;
use crate::classfile::class::ConstantPoolInfo;
use crate::classfile::class::MethodInfo;
use crate::classloader::class_loader;
use crate::common::error::Throwable;
use crate::common::param::DataType;
use crate::common::reference::Reference;
use crate::common::stack_frame;
use crate::common::stack_frame::create_stack_frame;
use crate::common::stack_frame::init_stack_frame;
use crate::common::stack_frame::StackFrame;
use crate::common::value::StackFrameValue;
use crate::native::native::run_native;
use crate::runtime::heap::Heap;
use crate::runtime::metaspace::Metaspace;
use crate::utils::debug::dprint;
use crate::utils::u8c::u8s_to_u16;
use log::info;
use log::warn;

pub fn invokespecial(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (target_class_name, method_name, descriptor) = {
        let this_class = &metaspace.classes[vm_stack[frame_index].class];
        let (class_index, name_and_type_index) = match &this_class.constant_pool[u8s_to_u16(
            &vm_stack[frame_index].code
                [(vm_stack[frame_index].pc + 1)..(vm_stack[frame_index].pc + 3)],
        ) as usize]
        {
            ConstantPoolInfo::Methodref(class_index, name_and_type_index) => {
                (class_index, name_and_type_index)
            }
            _ => panic!(),
        };
        // 通过链式调用减少嵌套
        let target_class_name = Some(&this_class.constant_pool[*class_index as usize])
            .and_then(|cp_info| match cp_info {
                ConstantPoolInfo::Class(name_index) => {
                    Some(&this_class.constant_pool[*name_index as usize])
                }
                _ => None,
            })
            .and_then(|name_info| match name_info {
                ConstantPoolInfo::Utf8(target_class_name) => Some(target_class_name),
                _ => None,
            });

        // 继续减少嵌套并简化逻辑
        let (method_name, descriptor) =
            match &this_class.constant_pool[*name_and_type_index as usize] {
                ConstantPoolInfo::NameAndType(name_index, descriptor_index) => {
                    let method_name = match &this_class.constant_pool[*name_index as usize] {
                        ConstantPoolInfo::Utf8(name) => name,
                        _ => panic!(),
                    };
                    let descriptor = match &this_class.constant_pool[*descriptor_index as usize] {
                        ConstantPoolInfo::Utf8(desc) => desc,
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
    let (method, class) =
        metaspace.get_method_from_root(&target_class_name, &method_name, &descriptor);
    //先移动
    vm_stack[frame_index].pc += 3;
    let method = method.unwrap();
    if method.access_flag & 0x0100 == 0 {
        let mut new_frame = init_stack_frame(&mut vm_stack[frame_index], &method, 1);
        let v = vm_stack[frame_index].op_stack.pop();
        match v {
            Some(obj) => {
                new_frame.local[0] = obj;
            }
            None => {
                panic!("error");
            }
        }
        vm_stack.push(new_frame);
    } else {
        run_native(&mut method.clone(), vm_stack, heap, metaspace);
    }
    Ok(())
}

pub fn invokeinterface(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index: usize = vm_stack.len() - 1;
    let mut new_frame = {
        let frame = &mut vm_stack[frame_index];
        //let class_name =metaspace.classes[];
        let cnt = frame.code[frame.pc + 3];
        let mut tmp: Vec<StackFrameValue> = Vec::new();
        for _i in 1..cnt {
            tmp.push(frame.op_stack.pop().unwrap());
        }
        let v = frame.op_stack.pop().unwrap();
        for _i in 1..cnt {
            frame.op_stack.push(tmp.pop().unwrap());
        }

        let (method_name, descriptor) = {
            let this_class: &mut class::Class = &mut metaspace.classes[frame.class];
            let (_class_index, name_and_type_index) = match &this_class.constant_pool
                [u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]) as usize]
            {
                ConstantPoolInfo::InterfaceMethodref(class_index, name_and_type_index) => {
                    (class_index, name_and_type_index)
                }
                _ => panic!(),
            };

            let (method_name, descriptor) = match &this_class.constant_pool
                [*name_and_type_index as usize]
            {
                ConstantPoolInfo::NameAndType(name_index, descriptor_index) => {
                    let method_name = match &this_class.constant_pool[*name_index as usize] {
                        ConstantPoolInfo::Utf8(name) => name,
                        _ => panic!(),
                    };
                    let descriptor = match &this_class.constant_pool[*descriptor_index as usize] {
                        ConstantPoolInfo::Utf8(desc) => desc,
                        _ => panic!(),
                    };
                    (method_name, descriptor)
                }
                _ => panic!(),
            };

            (method_name.clone(), descriptor.clone())
        };

        let class_name = {
            match v {
                StackFrameValue::Reference(id) => {
                    let class_id = heap.get_class(id);
                    metaspace.classes[class_id as usize].class_name.clone()
                }
                _ => panic!(),
            }
        };
        let (method, class) =
            metaspace.get_method_from_root(&class_name, &method_name, &descriptor);
        let mut new_frame: StackFrame = init_stack_frame(frame, &method.unwrap(), 1);
        new_frame.local[0] = v;
        new_frame
    };
    //push_stack_frame(new_frame);
    vm_stack.push(new_frame);
    vm_stack[frame_index].pc += 4;
    Ok(())
}

// pub fn invokevirtual(frame: &mut StackFrame) {
//     let method = frame.get_method_for_invoke().unwrap();
//     frame.pc += 3;
//     let param_len = method.param.len();
//     let sfv = frame
//         .op_stack
//         .get(frame.op_stack.len() - param_len - 1)
//         .unwrap();
//     let target_method = match sfv {
//         StackFrameValue::Reference(id) => {
//             let reference = get_reference(id).unwrap();
//             match reference {
//                 Reference::Object(object) => {
//                     let class_name = get_class_name(&object.class);
//                     let mut curr_class_name = class_name.clone();
//                     let mut target_method = get_method_from_pool(
//                         &curr_class_name,
//                         &method.method_name,
//                         &method.descriptor,
//                     );
//                     while target_method.is_none() {
//                         let clazz = get_or_load_class(&(curr_class_name.clone()));
//                         curr_class_name = clazz.super_class_name.clone();
//                         target_method = get_method_from_pool(
//                             &curr_class_name,
//                             &method.method_name,
//                             &method.descriptor,
//                         )
//                     }
//                     target_method
//                 }
//                 Reference::Array(_array) =>{
//                     Some(method)
//                 }
//             }
//         }
//         _ => panic!(),
//     };

//     let m= target_method.unwrap();
//     if m.access_flag & 0x0100 == 0 {
//         let mut new_frame = init_stack_frame(frame, &m, 1);
//         let v = frame.op_stack.pop();
//         match v {
//             Some(obj) => {
//                 new_frame.local[0] = obj;
//             }
//             None => {
//                 panic!("error");
//             }
//         }
//         push_stack_frame(new_frame);
//     } else {
//         run_native(&m, frame);
//     }
// }

pub fn invokevirtual(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (method_name, descriptor) = {
        let this_class = &metaspace.classes[vm_stack[frame_index].class];
        let (class_index, name_and_type_index) = match &this_class.constant_pool[u8s_to_u16(
            &vm_stack[frame_index].code
                [(vm_stack[frame_index].pc + 1)..(vm_stack[frame_index].pc + 3)],
        ) as usize]
        {
            ConstantPoolInfo::Methodref(class_index, name_and_type_index) => {
                (class_index, name_and_type_index)
            }
            _ => panic!(),
        };

        // 通过链式调用减少嵌套
        let target_class_name = Some(&this_class.constant_pool[*class_index as usize])
            .and_then(|cp_info| match cp_info {
                ConstantPoolInfo::Class(name_index) => {
                    Some(&this_class.constant_pool[*name_index as usize])
                }
                _ => None,
            })
            .and_then(|name_info| match name_info {
                ConstantPoolInfo::Utf8(target_class_name) => Some(target_class_name),
                _ => None,
            });

        // 继续减少嵌套并简化逻辑
        let (method_name, descriptor) =
            match &this_class.constant_pool[*name_and_type_index as usize] {
                ConstantPoolInfo::NameAndType(name_index, descriptor_index) => {
                    let method_name = match &this_class.constant_pool[*name_index as usize] {
                        ConstantPoolInfo::Utf8(name) => name,
                        _ => panic!(),
                    };
                    let descriptor = match &this_class.constant_pool[*descriptor_index as usize] {
                        ConstantPoolInfo::Utf8(desc) => desc,
                        _ => panic!(),
                    };
                    //
                    (method_name, descriptor)
                }
                _ => panic!(),
            };
        (method_name.clone(), descriptor.clone())
    };
    let parem_len = get_parameter_count(&descriptor);
    let sfv = vm_stack[frame_index].op_stack[vm_stack[frame_index].op_stack.len() - parem_len - 1];
    let method = {
        let mut method: Option<&MethodInfo> = None;
        match sfv {
            StackFrameValue::Reference(id) => {
               let class_id = heap.get_object_class_id(id as usize)?;
               let (m,c) =  metaspace.get_method_from_root(&metaspace.classes[class_id as usize].class_name.clone(), &method_name, &descriptor);
               method = m;
            }
            _ => {
                return Err(Throwable::Exception(
                    crate::common::error::Exception::NullPointer(
                        "Null pointer exception".to_string(),
                    ),
                ))
            }
        }

        if method.is_none() {
            return Err(Throwable::Exception(
                crate::common::error::Exception::NoSuchMethod(format!(
                    "no such method:{}",
                    method_name
                )),
            ));
        }

        method
    };
    let method = method.unwrap();
    if method.access_flag & 0x0100 == 0 {
        let mut new_frame = stack_frame::init_stack_frame(&mut vm_stack[frame_index], &method, 1);
        new_frame.local[0] = vm_stack[frame_index].op_stack.pop().unwrap();
        vm_stack.push(new_frame);
    } else {
        let _ = run_native(&mut method.clone(), vm_stack, heap, metaspace);
    }
    vm_stack[frame_index].pc += 3;
    Ok(())
}

pub fn invokestatic(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (target_class_name, method_name, descriptor) = {
        let this_class = &metaspace.classes[vm_stack[frame_index].class];
        let (class_index, name_and_type_index) = match &this_class.constant_pool[u8s_to_u16(
            &vm_stack[frame_index].code
                [(vm_stack[frame_index].pc + 1)..(vm_stack[frame_index].pc + 3)],
        ) as usize]
        {
            ConstantPoolInfo::Methodref(class_index, name_and_type_index) => {
                (class_index, name_and_type_index)
            }
            _ => panic!(),
        };

        // 通过链式调用减少嵌套
        let target_class_name = Some(&this_class.constant_pool[*class_index as usize])
            .and_then(|cp_info| match cp_info {
                ConstantPoolInfo::Class(name_index) => {
                    Some(&this_class.constant_pool[*name_index as usize])
                }
                _ => None,
            })
            .and_then(|name_info| match name_info {
                ConstantPoolInfo::Utf8(target_class_name) => Some(target_class_name),
                _ => None,
            });

        // 继续减少嵌套并简化逻辑
        let (method_name, descriptor) =
            match &this_class.constant_pool[*name_and_type_index as usize] {
                ConstantPoolInfo::NameAndType(name_index, descriptor_index) => {
                    let method_name = match &this_class.constant_pool[*name_index as usize] {
                        ConstantPoolInfo::Utf8(name) => name,
                        _ => panic!(),
                    };
                    let descriptor = match &this_class.constant_pool[*descriptor_index as usize] {
                        ConstantPoolInfo::Utf8(desc) => desc,
                        _ => panic!(),
                    };
                    //metaspace.get_method_from_root(class_name, method_name, descriptor)
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

    //确保加载
    class_loader::find_class(&target_class_name, vm_stack, heap, metaspace);
    let (method, class) =
        metaspace.get_method_from_root(&target_class_name, &method_name, &descriptor);
    let method = method.unwrap();
    //先移动
    vm_stack[frame_index].pc += 3;
    //我写的辅助调试输出的工具
    if method.method_name == "print20240503" {
        let v = vm_stack[frame_index].op_stack.pop().unwrap();
        dprint(v, vm_stack, heap, metaspace);
    } else if method.access_flag & 0x0100 == 0 {
        let new_frame = init_stack_frame(&mut vm_stack[frame_index], &method, 0);
        vm_stack.push(new_frame);
    } else {
        run_native(&method.clone(), vm_stack, heap, metaspace);
    }
    Ok(())
}

pub fn get_parameter_count(descriptor: &str) -> usize {
    let mut count = 0;
    let mut chars = descriptor.chars();

    if chars.next() != Some('(') {
        return 0;
    }

    let mut in_object = false;
    for c in chars {
        if c == ')' {
            break;
        }

        if in_object {
            if c == ';' {
                in_object = false;
                count += 1;
            }
            continue;
        }

        if c == '[' {
            continue;
        }

        if c == 'L' {
            in_object = true;
            continue;
        }

        match c {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => count += 1,
            _ => {}
        }
    }

    count
}
