pub mod op_code {
    use crate::class::ConstantPoolInfo;
    use crate::class::MethodInfo;
    use crate::reference::reference::Reference;
    use crate::runtime_data_area::create_array;
    use crate::runtime_data_area::create_object;
    use crate::runtime_data_area::get_class_name;
    use crate::runtime_data_area::get_method_from_pool;
    use crate::runtime_data_area::get_or_load_class;
    use crate::runtime_data_area::pop_stack_frame;
    use crate::runtime_data_area::push_frame_data;
    use crate::runtime_data_area::VM_STACKS;
    use crate::stack_frame::init_stack_frame;
    use crate::stack_frame::StackFrame;
    use crate::u8c::u8s_to_u16;
    use crate::value::value::StackFrameValue;
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    extern crate env_logger;
    extern crate log;
    use crate::opcode_array::*;
    use crate::opcode_load::*;
    use crate::opcode_push::*;
    use crate::opcode_store::*;
    use crate::opcode_nop::*;
    use crate::opcode_const::*;
    use crate::opcode_ldc::*;
    use crate::opcode_dup::*;
    use crate::opcode_return::*;
    use crate::opcode_math::*;
    use crate::opcode_pop::*;
    use crate::opcode_swap::*;
    use crate::param::param::MethodParameter;
    use log::{error, info, warn};
    use std::env;

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while vm_stack.len() > 0 {
            let mut len = (&vm_stack).len();
            let mut stack_frame = vm_stack.get_mut(len - 1).unwrap();
            while stack_frame.pc < stack_frame.code.len() {
                let code = stack_frame.code[stack_frame.pc];
                match code {
                    0x00 => nop(stack_frame),
                    0x01 => aconst_null(stack_frame),
                    0x02 => iconst_m1(stack_frame),
                    0x03 => iconst_0(stack_frame),
                    0x04 => iconst_1(stack_frame),
                    0x05 => iconst_2(stack_frame),
                    0x06 => iconst_3(stack_frame),
                    0x07 => iconst_4(stack_frame),
                    0x08 => iconst_5(stack_frame),
                    0x09 => lconst_0(stack_frame),
                    0x0a => lconst_1(stack_frame),
                    0x0b => fconst_0(stack_frame),
                    0x0c => fconst_1(stack_frame),
                    0x0d => fconst_2(stack_frame),
                    0x0e => dconst_0(stack_frame),
                    0x0f => dconst_1(stack_frame),
                    0x10 => bipush(stack_frame),
                    0x11 => sipush(stack_frame),
                    0x12 => ldc(stack_frame),
                    0x13 => ldc_w(stack_frame),
                    0x14 => ldc2_w(stack_frame),
                    0x15 => iload(stack_frame),
                    0x16 => lload(stack_frame),
                    0x17 => fload(stack_frame),
                    0x18 => dload(stack_frame),
                    0x19 => aload(stack_frame),
                    0x1a => iload_0(stack_frame),
                    0x1b => iload_1(stack_frame),
                    0x1c => iload_2(stack_frame),
                    0x1d => iload_3(stack_frame),
                    0x1e => lload_0(stack_frame),
                    0x1f => lload_1(stack_frame),
                    0x20 => lload_2(stack_frame),
                    0x21 => lload_3(stack_frame),
                    0x22 => fload_0(stack_frame),
                    0x23 => fload_1(stack_frame),
                    0x24 => fload_2(stack_frame),
                    0x25 => fload_3(stack_frame),
                    0x26 => dload_0(stack_frame),
                    0x27 => dload_1(stack_frame),
                    0x28 => dload_2(stack_frame),
                    0x29 => dload_3(stack_frame),
                    0x2a => aload_0(stack_frame),
                    0x2b => aload_1(stack_frame),
                    0x2c => aload_2(stack_frame),
                    0x2d => aload_3(stack_frame),
                    0x2e => iaload(stack_frame),
                    0x2f => laload(stack_frame),
                    0x30 => faload(stack_frame),
                    0x31 => daload(stack_frame),
                    0x32 => aaload(stack_frame),
                    0x33 => baload(stack_frame),
                    0x34 => caload(stack_frame),
                    0x35 => saload(stack_frame),
                    0x36 => istore(stack_frame),
                    0x37 => lstore(stack_frame),
                    0x38 => fstore(stack_frame),
                    0x39 => dstore(stack_frame),
                    0x3a => astore(stack_frame),
                    0x3b => istore_0(stack_frame),
                    0x3c => istore_1(stack_frame),
                    0x3d => istore_2(stack_frame),
                    0x3e => istore_3(stack_frame),
                    0x3f => lstore_0(stack_frame),
                    0x40 => lstore_1(stack_frame),
                    0x41 => lstore_2(stack_frame),
                    0x42 => lstore_3(stack_frame),
                    0x43 => fstore_0(stack_frame),
                    0x44 => fstore_1(stack_frame),
                    0x45 => fstore_2(stack_frame),
                    0x46 => fstore_3(stack_frame),
                    0x47 => dstore_0(stack_frame),
                    0x48 => dstore_1(stack_frame),
                    0x49 => dstore_2(stack_frame),
                    0x4a => dstore_3(stack_frame),
                    0x4b => astore_0(stack_frame),
                    0x4c => astore_1(stack_frame),
                    0x4d => astore_2(stack_frame),
                    0x4e => astore_3(stack_frame),
                    0x4f => iastore(stack_frame),
                    0x50 => lastore(stack_frame),
                    0x51 => fastore(stack_frame),
                    0x52 => dastore(stack_frame),
                    0x53 => aastore(stack_frame),
                    0x54 => bastore(stack_frame),
                    0x55 => castore(stack_frame),
                    0x56 => sastore(stack_frame),
                    0x57 => pop(stack_frame),
                    0x58 => pop2(stack_frame),
                    0x59 => dup(stack_frame),
                    0x5a => dup_x1(stack_frame),
                    0x5b => dup_x2(stack_frame),
                    0x5c => dup2(stack_frame),
                    0x5d => dup2_x1(stack_frame),
                    0x5e => dup2_x2(stack_frame),
                    0x5f => swap(stack_frame),
                    0x60 => iadd(stack_frame),
                    0x61 => ladd(stack_frame),
                    0x62 => fadd(stack_frame),
                    0x63 => dadd(stack_frame),
                    0x64 => isub(stack_frame),
                    0x65 => lsub(stack_frame),
                    0x66 => fsub(stack_frame),
                    0x67 => dsub(stack_frame),
                    0x68 => imul(stack_frame),
                    0x69 => lmul(stack_frame),
                    0x6a => fmul(stack_frame),
                    0x6b => dmul(stack_frame),
                    0x6c => idiv(stack_frame),
                    0x6d => ldiv(stack_frame),
                    0x6e => fdiv(stack_frame),
                    0x6f => ddiv(stack_frame),
                    // 0x70 => irem(stack_frame),
                    // 0x71 => lrem(stack_frame),
                    // 0x72 => frem(stack_frame),
                    // 0x73 => drem(stack_frame),
                    // 0x74 => ineg(stack_frame),
                    // 0x75 => lneg(stack_frame),
                    // 0x76 => fneg(stack_frame),
                    // 0x77 => dneg(stack_frame),
                    // 0x78 => ishl(stack_frame),
                    // 0x79 => lshl(stack_frame),
                    // 0x7a => ishr(stack_frame),
                    // 0x7b => lshr(stack_frame),
                    // 0x7c => iushr(stack_frame),
                    // 0x7d => lushr(stack_frame),
                    // 0x7e => iand(stack_frame),
                    // 0x7f => land(stack_frame),
                    // 0x80 => ior(stack_frame),
                    // 0x81 => lor(stack_frame),
                    // 0x82 => ixor(stack_frame),
                    // 0x83 => lxor(stack_frame),
                    // 0x84 => iinc(stack_frame),
                    // 0x85 => i2l(stack_frame),
                    // 0x86 => i2f(stack_frame),
                    // 0x87 => i2d(stack_frame),
                    // 0x88 => l2i(stack_frame),
                    // 0x89 => l2f(stack_frame),
                    // 0x8a => l2d(stack_frame),
                    // 0x8b => f2i(stack_frame),
                    // 0x8c => f2l(stack_frame),
                    // 0x8d => f2d(stack_frame),
                    // 0x8e => d2i(stack_frame),
                    // 0x8f => d2l(stack_frame),
                    // 0x90 => d2f(stack_frame),
                    // 0x91 => i2b(stack_frame),
                    // 0x92 => i2c(stack_frame),
                    // 0x93 => i2s(stack_frame),
                    // 0x94 => lcmp(stack_frame),
                    // 0x95 => fcmpl(stack_frame),
                    // 0x96 => fcmpg(stack_frame),
                    // 0x97 => dcmpl(stack_frame),
                    // 0x98 => dcmpg(stack_frame),
                    // 0x99 => ifeq(stack_frame),
                    // 0x9a => ifne(stack_frame),
                    // 0x9b => iflt(stack_frame),
                    // 0x9c => ifge(stack_frame),
                    // 0x9d => ifgt(stack_frame),
                    // 0x9e => ifle(stack_frame),
                    // 0x9f => if_icmpeq(stack_frame),
                    // 0xa0 => if_icmpne(stack_frame),
                    // 0xa1 => if_icmplt(stack_frame),
                    // 0xa2 => if_icmpge(stack_frame),
                    // 0xa3 => if_icmpgt(stack_frame),
                    // 0xa4 => if_icmple(stack_frame),
                    // 0xa5 => if_acmpeq(stack_frame),
                    // 0xa6 => if_acmpne(stack_frame),
                    // 0xa7 => goto(stack_frame),
                    // 0xa8 => jsr(stack_frame),
                    // 0xa9 => ret(stack_frame),
                    // 0xaa => tableswitch(stack_frame),
                    // 0xab => lookupswitch(stack_frame),
                    0xac => ireturn(stack_frame),
                    // 0xad => lreturn(stack_frame),
                    // 0xae => freturn(stack_frame),
                    // 0xaf => dreturn(stack_frame),
                    // 0xb0 => areturn(stack_frame),
                    0xb1 => _return (stack_frame),
                    // 0xb2 => getstatic(stack_frame),
                    // 0xb3 => putstatic(stack_frame),
                    // 0xb4 => getfield(stack_frame),
                    // 0xb5 => putfield(stack_frame),
                    0xb6 => invokevirtual(stack_frame),
                    0xb7 => invokespecial(stack_frame),
                    // 0xb8 => invokestatic(stack_frame),
                    // 0xb9 => invokeinterface(stack_frame),
                    // 0xba => invokedynamic(stack_frame),
                    0xbb => _new(stack_frame),
                    0xbc => newarray(stack_frame),
                    // 0xbd => anewarray(stack_frame),
                    // 0xbe => arraylength(stack_frame),
                    // 0xbf => athrow(stack_frame),
                    // 0xc0 => checkcast(stack_frame),
                    // 0xc1 => instanceof(stack_frame),
                    // 0xc2 => monitorenter(stack_frame),
                    // 0xc3 => monitorexit(stack_frame),
                    // 0xc4 => wide(stack_frame),
                    // 0xc5 => multianewarray(stack_frame),
                    // 0xc6 => ifnull(stack_frame),
                    // 0xc7 => ifnonnull(stack_frame),
                    // 0xc8 => goto_w(stack_frame),
                    // 0xc9 => jsr_w(stack_frame),
                    _ => {
                        // 处理未知指令的情况，可以抛出错误或执行默认操作
                        panic!("Unknown instruction code: 0x{:02X}", code);
                    }
                }
                len = (&vm_stack).len();
                if len == 0 {
                    break;
                }
                stack_frame = vm_stack.get_mut(len - 1).unwrap();
            }
        }
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
        let classfile_pool_class = &this_class.constant_pool.get(&classfile_pool_index).unwrap();
        match classfile_pool_class {
            ConstantPoolInfo::Class(name_index) => {
                let class_name_utf8 = &this_class.constant_pool.get(name_index).unwrap();
                match class_name_utf8 {
                    ConstantPoolInfo::Utf8(class_name) => {
                        let target_class = get_or_load_class(class_name);
                        let obj = create_object(target_class.id);
                        match obj {
                            Reference::Object(object) => {
                                frame.op_stack.push(StackFrameValue::Reference(object.id));
                                frame.pc += 3;
                            }
                            _ => panic!("wrong object data"),
                        }
                    }
                    _ => panic!("wrong class data"),
                }
            }
            _ => panic!("wrong class data"),
        }
    }

    pub fn get_method_for_invoke(frame: &StackFrame) -> Option<&MethodInfo> {
        let class_name = get_class_name(&frame.class);
        let this_class = get_or_load_class(&class_name);
        if let ConstantPoolInfo::Methodref(class_index, name_and_type_index) = &this_class
            .constant_pool.get(&u8s_to_u16(&frame.code[(frame.pc + 1)..(frame.pc + 3)])).unwrap()
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
}
