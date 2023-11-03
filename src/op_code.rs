pub mod op_code {
    use crate::class::class::Class;
    use crate::class::class::ConstantPoolInfo;
    use crate::class::class::MethodInfo;
    use crate::class_loader::class_loader::get_method;
    use crate::runtime_data_area::runtime_data_area::VM_STACKS;
    use crate::stack_frame::stack_frame::StackFrame;
    use crate::stack_frame::stack_frame::init_stack_frame;

    use crate::u8c::u8c::u8s_to_u16;
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use crate::value::value::StackFrameValue;
    use crate::runtime_data_area::runtime_data_area::get_class_name;
    use crate::runtime_data_area::runtime_data_area::get_or_load_class;
    use crate::runtime_data_area::runtime_data_area::create_object;
    use crate::runtime_data_area::runtime_data_area::get_method_from_pool;

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while vm_stack.len() > 0 {
            let mut len = (&vm_stack).len();
            let mut stack_frame = vm_stack.get_mut(len - 1).unwrap();
            while stack_frame.pc < stack_frame.code.len() {
                let code = stack_frame.code[stack_frame.pc];
                //println!("op code:{}{:X},{:?}",stack_frame.vm_stack_id,&code,stack_frame.op_stack.get(0));
                println!("frame:{:?}",&stack_frame);
               // println!("vm_stack:{:?}",&vm_stack.clone());
                if code == 0x10 {
                    bipush(stack_frame);
                } else if code == 0x3c {
                    istore_1(stack_frame);
                } else if code == 0x3d {
                    istore_2(stack_frame);
                } else if code == 0x3e {
                    istore_3(stack_frame);
                } else if code == 0x1b {
                    iload_1(stack_frame);
                } else if code == 0x2a {
                    aload_0(stack_frame);
                } else if code == 0x2b {
                    aload_1(stack_frame);
                } else if code == 0x1c {
                    iload_2(stack_frame);
                } else if code == 0x60 {
                    iadd(stack_frame);
                } else if code == 0x11 {
                    sipush(stack_frame);
                } else if code == 0xb1 {
                    _return(stack_frame);
                } else if code == 0xac {
                    ireturn(stack_frame);
                } else if code == 0x59 {
                    dup(stack_frame);
                } else if code == 0xbb {
                    _new(stack_frame);
                } else if code == 0xb7 {
                    invokespecial(stack_frame);
                } else if code == 0xb6 {
                    invokevirtual(stack_frame);
                } else if code == 0x4c {
                    astore_1(stack_frame);
                }
                len = (&vm_stack).len();
                if len == 0 {
                    break;
                }
                stack_frame = vm_stack.get_mut(len - 1).unwrap();
            }
        }
    }

    pub fn bipush(frame: &mut StackFrame) {
        let u = frame.code[frame.pc + 1];
        frame.op_stack.push(StackFrameValue::Byte(u as i8));
        frame.pc += 2;
    }

    pub fn sipush(frame: &mut StackFrame) {
        frame
            .op_stack
            .push(StackFrameValue::Short(u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]) as i16));
        frame.pc += 3;
    }

    pub fn istore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn istore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }

    pub fn istore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }

    pub fn astore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn aload_1(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local.get(1).unwrap().clone());
        frame.pc += 1;
    }

    pub fn aload_0(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local.get(0).unwrap().clone());
        frame.pc += 1;
    }

    pub fn iload_1(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local.get(1).unwrap().clone());
        frame.pc += 1;
    }

    pub fn iload_2(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local.get(2).unwrap().clone());
        frame.pc += 1;
    }

    pub fn iadd(frame: &mut StackFrame) {
        let i1 = frame.popu64() as i32;
        let i2 = frame.popu64() as i32;
        let result = i1 + i2;
        println!("execute add result: {}", &result);
        frame.op_stack.push(StackFrameValue::Int(result));
        frame.pc += 1;
    }

    // pub fn ldc( frame: &mut StackFrame,  str_pool: &mut HashMap<Vec<u8>, Object>) {
    //     let u = frame.code[frame.pc + 1];
    //     if let Some(constant) = frame.class.constant_pool.get((u - 1) as usize) {
    //         // Integer Float
    //         if constant[0] == 0x3 || constant[0] == 0x4 {
    //             frame.op_stack.push(classfile::u8s_to_u32(&constant[1..5]) as u64);
    //         } else if constant[0] == 8 {
    //             // 判断字符串常量池中是否有，没有的话就创建一个
    //             let str_code = frame
    //                 .class
    //                 .constant_pool
    //                 .get(classfile::u8s_to_u16(&constant[1..2]) as usize)
    //                 .unwrap()
    //                 .to_vec();
    //             if str_pool.contains_key(&str_code) {
    //                 //frame.op_stack.push(str_pool.get(&str_code).unwrap())
    //             } else {
    //                 //创建一个String对象
    //             }
    //         }
    //     }
    // }

    pub fn _new(frame: &mut StackFrame) {
        let class_name = get_class_name(&frame.class);
        let this_class = get_or_load_class(&class_name);
        let classfile_pool_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]);
        let classfile_pool_class = &this_class.constant_pool[(classfile_pool_index - 1) as usize];
        match classfile_pool_class {
            ConstantPoolInfo::Class(name_index) =>{
                let class_name_utf8 = &this_class.constant_pool[(*name_index - 1) as usize];
                match class_name_utf8 {
                    ConstantPoolInfo::Utf8(class_name) =>{
                        let target_class = get_or_load_class(class_name);
                        let obj = create_object(target_class.id);
                        frame.op_stack.push(StackFrameValue::Reference(obj.id));
                        frame.pc += 3;
                    }
                    _=> panic!("wrong class data")
                }
            }
            _=> panic!("wrong class data")
        }    
      
    }

    // pub fn get_class_for_invoke(frame: &StackFrame) -> usize {
    //     let class_name = get_class_name(&frame.class);
    //     let this_class = get_or_load_class(&class_name);
    //     let method_ref_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]) as usize;
    //     let method_ref = &this_class.constant_pool[method_ref_index - 1 as usize];
    //     match method_ref {
    //         ConstantPoolInfo::Methodref(class_index,name_and_type_index ) =>{
    //             let constant_class = this_class.constant_pool[(*class_index - 1) as usize];
    //             match constant_class {
    //                 ConstantPoolInfo::Class(name_index) =>{
    //                     let class_utf8 = &this_class.constant_pool[(name_index - 1) as usize];
    //                     match class_utf8 {
    //                         ConstantPoolInfo::Utf8(class_name) =>{
    //                             let class: &mut Class = get_or_load_class(&class_name);
    //                             return  class.id
    //                         }
    //                         _=> panic!("wrong class data")
    //                     }
    //                 }
    //                 _=> panic!("wrong class data")
    //             }
    //         }
    //         _=> panic!("wrong class data")
    //     }
    // }



/*
    pub fn get_method_for_invoke(frame: &StackFrame) -> Option<&MethodInfo> {
        let class_name = get_class_name(&frame.class);
        let this_class = get_or_load_class(&class_name);
        let method_ref_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]) as usize;
        let method_ref = &this_class.constant_pool[method_ref_index - 1 as usize];
        match method_ref {
            ConstantPoolInfo::Methodref(class_index,name_and_type_index ) =>{
                let name_and_type =  this_class.constant_pool[(*name_and_type_index - 1) as usize];
                let constant_class = this_class.constant_pool[(*class_index - 1) as usize];
                match constant_class {
                    ConstantPoolInfo::Class(name_index) =>{
                        let class_utf8 = &this_class.constant_pool[(name_index - 1) as usize];
                        match class_utf8 {
                            ConstantPoolInfo::Utf8(class_name) =>{
                                let target_class: &mut Class = get_or_load_class(&class_name);
                                match name_and_type {
                                    ConstantPoolInfo::NameAndType(name_index, descriptor_index) =>{
                                        let method_name_utf8 = &this_class.constant_pool[(name_index - 1) as usize];
                                        match method_name_utf8 {
                                            ConstantPoolInfo::Utf8(method_name) =>{
                                                let method_descriptor = &this_class.constant_pool[(name_index - 1) as usize];
                                                match method_descriptor {
                                                    ConstantPoolInfo::Utf8(descriptor) =>{
                                                       return Some( get_method_from_pool(target_class.class_name,method_name.clone(),descriptor.clone()))
                                                    }
                                                    _=> panic!("wrong class data")
                                                }
                                            }
                                            _=> panic!("wrong class data")
                                        }
                                    }
                                    _=> panic!("wrong class data")
                                }
                            }
                            _=> panic!("wrong class data")
                        }
                    }
                    _=> panic!("wrong class data")
                }
            }
            _=> panic!("wrong class data")
        }

    }
    */


    pub fn get_method_for_invoke(frame: &StackFrame) -> Option<&MethodInfo> {
        let class_name = get_class_name(&frame.class);
        let this_class = get_or_load_class(&class_name);
        if let ConstantPoolInfo::Methodref(class_index, name_and_type_index) = &this_class.constant_pool[u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]) as usize - 1] {
            if let ConstantPoolInfo::Class(name_index) = &this_class.constant_pool[*class_index as usize - 1] {
                if let ConstantPoolInfo::Utf8(class_name) = &this_class.constant_pool[*name_index as usize - 1] {
                    let target_class = get_or_load_class(&class_name);
                    if let ConstantPoolInfo::NameAndType(name_index, descriptor_index) = &this_class.constant_pool[*name_and_type_index as usize - 1] {
                        if let ConstantPoolInfo::Utf8(method_name) = &this_class.constant_pool[*name_index as usize - 1] {
                            if let ConstantPoolInfo::Utf8(descriptor) = &this_class.constant_pool[*descriptor_index as usize - 1] {
                                return Some(get_method_from_pool(target_class.class_name.clone(), method_name.clone(), descriptor.clone()));
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
        let new_frame = init_stack_frame(frame, method.unwrap());
        push_stack_frame(new_frame);
        frame.pc += 3;
    }

    pub fn invokevirtual(frame: &mut StackFrame) {
        let clone_frame = &frame.clone();
        // let target_class_id = get_class_for_invoke(&frame.clone());
        let method = get_method_for_invoke(&clone_frame);
        let mut new_frame = init_stack_frame(frame, method.unwrap());
        let v = frame.op_stack.pop();
        match v {
            Some(obj) =>{
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
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            if stack_frame.vm_stack_id == 0 {
                for i in 1..0xFFFFFFF as u32 {
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

    pub fn pop_stack_frame(vm_stack_id: u32) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
            VM_STACKS.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            map.get_mut(&vm_stack_id).unwrap().pop();
        }
        drop(data);
    }

    pub fn push_frame_data(vm_stack_id: u32, value:StackFrameValue) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
            VM_STACKS.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            println!("before push_frame_data：{:?}",&map);
            let l = map.get_mut(&vm_stack_id).unwrap();
            //let len = l.len();
            l.get_mut(0).unwrap().op_stack.push(value);
            println!("after push_frame_data：{:?}",&map);
        }
        drop(data);
    }

    pub fn dup(frame: &mut StackFrame) {
        frame.op_stack.push(frame.op_stack[0].clone());
        frame.pc += 1;
    }

    pub fn _return(frame: &mut StackFrame) {
        pop_stack_frame(frame.vm_stack_id);
        frame.pc += 1;
    }

    pub fn ireturn(frame: &mut StackFrame) {
        let v: StackFrameValue = frame.op_stack.pop().unwrap();
        println!("ireturn result: {:?}", &v);
        pop_stack_frame(frame.vm_stack_id);
        push_frame_data(frame.vm_stack_id, v);
        //将返回值传给上一个栈帧
        frame.pc += 1;
    }
}
