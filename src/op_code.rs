pub mod op_code {
    use crate::class::class::Class;
    use crate::class::class::CodeAttribute;
    use crate::class::class::MethodInfo;
    use crate::class::class::MethodParameter;
    use crate::class_loader::class_loader::load_class;
    use crate::create_stack_frame;
    use crate::object::object::Object;
    use crate::runtime_data_area::runtime_data_area::CLASS_DATA;
    use crate::runtime_data_area::runtime_data_area::CLASS_ID_DATA;
    use crate::runtime_data_area::runtime_data_area::OBJECT_DATA;
    use crate::runtime_data_area::runtime_data_area::OBJECT_ID;
    use crate::runtime_data_area::runtime_data_area::STR_POOL;
    use crate::runtime_data_area::runtime_data_area::VM_STACKS;
    use crate::stack_frame;
    use crate::stack_frame::stack_frame::StackFrame;
    use crate::u8c::u8c::u8s_to_u16;
    use crate::u8c::u8c::u8s_to_u32;
    use crate::u8c::u8c::u8s_to_u64;
    use lazy_static::lazy_static;
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use std::mem;
    use std::sync::Mutex;

    pub fn class_exists(class_name: Vec<u8>) -> bool {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<Vec<u8>, Class>>> =
            CLASS_DATA.lock().unwrap();
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            drop(data);
            return map.contains_key(&class_name);
        }
    }

    pub fn create_object<'a>(class: usize) -> &'a mut Object {
        // 获取全局变量的Mutex锁
        let data = OBJECT_DATA.lock().unwrap();
        let obj_id_data = OBJECT_ID.lock().unwrap();

        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        let obj;
        let obj_id: &mut u64;
        unsafe {
            let map = &mut *data.get();
            obj_id = &mut *obj_id_data.get();
            obj = Object::new(*obj_id + 1, class);
            map.insert(obj.id, obj.clone());
            drop(data);
            drop(obj_id_data);
            *obj_id += 1;
            return map.get_mut(&obj.id).unwrap();
        }
    }

    pub fn get_object<'a>(id: &u64) -> &'a mut Object {
        // 获取全局变量的Mutex锁
        let data = OBJECT_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            let map = &mut *data.get();
            return map.get_mut(id).unwrap();
        }
    }
    pub fn get_or_load_class<'a>(class_name: &Vec<u8>) -> &'a mut Class {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<Vec<u8>, Class>>> =
            CLASS_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            if (!map.contains_key(class_name)) {
                let mut class = load_class(&(String::from_utf8(class_name.clone()).unwrap()));
                class.id = map.len() + 1 as usize;
                map.insert(class_name.clone(), class);
                add_id_class(map.len(), class_name.clone());
            }
            drop(data);
            return map.get_mut(class_name).unwrap();
        }
    }

    pub fn add_id_class(class_id: usize, class_name: Vec<u8>) {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, Vec<u8>>>> =
            CLASS_ID_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 添加或修改键值对
            map.insert(class_id, class_name);
        }
        // 释放Mutex锁
        drop(data);
    }

    pub fn get_class_name(class_id: &usize) -> Vec<u8> {
        // 获取全局变量的Mutex锁
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, Vec<u8>>>> =
            CLASS_ID_DATA.lock().unwrap();
        // 通过UnsafeCell获取可变引用，并修改全局变量的值
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            // 释放Mutex锁
            drop(data);
            return map.get(class_id).unwrap().clone();
        }
    }

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while vm_stack.len() > 0 {
            let mut len = (&vm_stack).len();
            let mut stack_frame = vm_stack.get_mut(len - 1).unwrap();
            while stack_frame.pc < stack_frame.code.len() {
                let code = stack_frame.code[stack_frame.pc];
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
                if (len == 0) {
                    break;
                }
                stack_frame = vm_stack.get_mut(len - 1).unwrap();
            }
        }
    }

    pub fn bipush(frame: &mut StackFrame) {
        let u = frame.code[frame.pc + 1];
        frame.op_stack.push(u as u64);
        frame.pc += 2;
    }

    pub fn sipush(frame: &mut StackFrame) {
        frame
            .op_stack
            .push(u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]) as u64);
        frame.pc += 3;
    }

    pub fn istore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap() as u64;
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn istore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i as u64;
        frame.pc += 1;
    }

    pub fn istore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap() as u64;
        frame.local[3] = i;
        frame.pc += 1;
    }

    pub fn astore_1(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        frame.local[1] = v;
        frame.pc += 1;
    }

    pub fn aload_1(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local[1]);
        frame.pc += 1;
    }

    pub fn aload_0(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local[0]);
        frame.pc += 1;
    }

    pub fn iload_1(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local[1]);
        frame.pc += 1;
    }

    pub fn iload_2(frame: &mut StackFrame) {
        frame.op_stack.push(frame.local[2]);
        frame.pc += 1;
    }

    pub fn iadd(frame: &mut StackFrame) {
        let i1 = frame.op_stack.pop().unwrap() as i32;
        let i2 = frame.op_stack.pop().unwrap() as i32;
        let result = i1 + i2;
        println!("add result: {}", &result);
        frame.op_stack.push(result as u64);
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
        let class = get_or_load_class(&class_name);
        let classfile_pool_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]);
        let classfile_pool_class = &class.constant_pool[(classfile_pool_index - 1) as usize];
        let classfile_class_name_index = u8s_to_u16(&classfile_pool_class[1..3]);
        let class_name_utf8 = &class.constant_pool[(classfile_class_name_index - 1) as usize];
        let len = u8s_to_u16(&class_name_utf8[1..3]);
        let class = get_or_load_class(&class_name_utf8[3..(len as usize + 3)].to_vec());
        let obj = create_object(class.id);
        frame.op_stack.push(obj.id);
        frame.pc += 3;
    }

    pub fn get_class_for_invoke(frame: &StackFrame) -> usize {
        let class_name = get_class_name(&frame.class);
        let this_class = get_or_load_class(&class_name);
        let method_ref_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]) as usize;
        let method_ref = &this_class.constant_pool[method_ref_index - 1 as usize];
        let class_index = u8s_to_u16(&method_ref[1..3]) as usize;
        let constant_class = &this_class.constant_pool[class_index - 1 as usize];
        let class_name_index = u8s_to_u16(&constant_class[1..3]) as usize;
        let class_utf8 = &this_class.constant_pool[class_name_index - 1 as usize];
        let class_name_length = u8s_to_u16(&class_utf8[1..3]) as usize;
        let class_name = &class_utf8[3..(3 + class_name_length)];
        let class: &mut Class = get_or_load_class(&class_name.to_vec());
        return class.id;
    }

    pub fn get_method_for_invoke(frame: &StackFrame) -> Option<&MethodInfo> {
        let class_name = get_class_name(&frame.class);
        let this_class = get_or_load_class(&class_name);
        let method_ref_index = u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)]) as usize;
        let method_ref = &this_class.constant_pool[method_ref_index - 1 as usize];
        let class_index = u8s_to_u16(&method_ref[1..3]) as usize;
        let constant_class = &this_class.constant_pool[class_index - 1 as usize];
        let class_name_index = u8s_to_u16(&constant_class[1..3]) as usize;
        let class_utf8 = &this_class.constant_pool[class_name_index - 1 as usize];
        let class_name_length = u8s_to_u16(&class_utf8[1..3]) as usize;
        let class_name = &class_utf8[3..(3 + class_name_length)];
        let class: &mut Class = get_or_load_class(&class_name.to_vec());
        let name_and_type_index: usize = u8s_to_u16(&method_ref[3..5]) as usize;
        let name_and_type = &this_class.constant_pool[name_and_type_index - 1 as usize];
        let name_index: usize = u8s_to_u16(&name_and_type[1..3]) as usize;
        let name_utf8 = &this_class.constant_pool[name_index - 1];
        let descreptor_index: usize = u8s_to_u16(&name_and_type[3..5]) as usize;
        let descreptor_utf8 = &this_class.constant_pool[descreptor_index - 1];
        for i in 0..class.methods_count as usize {
            let m = &class.method_info[i];
            //m.access_flag
            //目的是找到这个方法
            let name = &class.constant_pool[(m.name_index - 1) as usize];
            let description = &class.constant_pool[(m.descriptor_index - 1) as usize];
            if name == name_utf8 && descreptor_utf8 == description {
                return Some(m);
            }
        }
        return None;
    }

    //对象的初始化方法
    pub fn invokespecial(frame: &mut StackFrame) {
        let clone_frame = &frame.clone();
        let target_class_id = get_class_for_invoke(&frame.clone());
        let method = get_method_for_invoke(&clone_frame);
        let new_frame = init_stack_frame(frame, method.unwrap(), target_class_id);
        push_stack_frame(new_frame);
        frame.pc += 3;
    }

    pub fn invokevirtual(frame: &mut StackFrame) {
        let clone_frame = &frame.clone();
        let target_class_id = get_class_for_invoke(&frame.clone());
        let method = get_method_for_invoke(&clone_frame);
        let mut new_frame = init_stack_frame(frame, method.unwrap(), target_class_id);
        let v = frame.op_stack.pop();
        match v {
            Some(obj) =>{
                new_frame.local[0] = frame.op_stack.pop().unwrap();
            }
            None => {
                panic!("error");
            }
        }
        //new_frame.local[0] = frame.op_stack.pop().unwrap();
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

    pub fn push_frame_data(vm_stack_id: u32, value: u64) {
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
            VM_STACKS.lock().unwrap();
        unsafe {
            let map = &mut *data.get();
            let l = map.get_mut(&vm_stack_id).unwrap();
            let len = l.len();
            l.get_mut(len - 1).unwrap().op_stack.push(value);
        }
        drop(data);
    }

    pub fn init_stack_frame(
        frame: &mut StackFrame,
        method_info: &MethodInfo,
        class_id: usize,
    ) -> StackFrame {
        let mut new_stack_frame: StackFrame = create_stack_frame(&method_info, class_id).unwrap();
        new_stack_frame.vm_stack_id = frame.vm_stack_id;
        let mut i: usize = 0;
        if method_info.param.len() > 0 {
            for j in 0..&method_info.param.len() - 1 {
                let v = frame.op_stack.pop().unwrap();
                let param = method_info.param.get(j).unwrap();
                match param {
                    MethodParameter::Byte => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Char => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Array {
                        element_type,
                        depth,
                    } => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Boolean => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Double => {
                        let bytes: [u8; 8] = unsafe { mem::transmute(v) };
                        new_stack_frame.local[i + 1] = v;
                        new_stack_frame.local[i + 2] = v;
                        i += 2;
                    }
                    MethodParameter::Float => {
                        new_stack_frame.local.push(v);
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Int => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Long => {
                        let bytes: [u8; 8] = unsafe { mem::transmute(v) };
                        new_stack_frame.local[i + 1] = (u8s_to_u32(&bytes[0..4]) as u64);
                        new_stack_frame.local[i + 2] = (u8s_to_u32(&bytes[4..8]) as u64);
                        i += 2;
                    }
                    MethodParameter::Reference(string) => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                    MethodParameter::Short => {
                        new_stack_frame.local[i + 1] = v;
                        i += 1;
                    }
                }
            }
        }
        return new_stack_frame;
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
        let v = frame.op_stack.pop().unwrap();
        println!("ireturn result: {}", &v);
        pop_stack_frame(frame.vm_stack_id);
        push_frame_data(frame.vm_stack_id, v);
        //将返回值传给上一个栈帧
        frame.pc += 1;
    }
}
