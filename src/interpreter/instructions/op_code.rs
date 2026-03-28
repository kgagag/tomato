pub mod op_code {
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use std::time::Instant;
    use std::{mem, result};

    use log::{error, info, warn};
    use opcode_array::*;
    use opcode_checkcast::*;
    use opcode_compare::*;
    use opcode_const::*;
    use opcode_convert::*;
    use opcode_dup::*;
    use opcode_exception::*;
    use opcode_field::*;
    use opcode_goto::*;
    use opcode_instanceof::*;
    use opcode_invoke::*;
    use opcode_ldc::*;
    use opcode_load::*;
    use opcode_math::*;
    use opcode_new::*;
    use opcode_nop::*;
    use opcode_pop::*;
    use opcode_push::*;
    use opcode_return::*;
    use opcode_static::*;
    use opcode_store::*;
    use opcode_swap::*;
    use opcode_thread::*;

    use crate::classfile::class::{AttributeInfo, Class, ConstantPoolInfo, Exception, MethodInfo};
    use crate::classloader;
    use crate::common::error::Throwable;
    use crate::common::stack_frame::{self, StackFrame};
    use crate::common::value::StackFrameValue;
    use crate::interpreter::instructions::*;
    use crate::runtime::heap::Heap;
    use crate::runtime::metaspace::Metaspace;
    use crate::runtime::vm;
    use crate::runtime::vm::Vm;
    use opcode_nop::nop;
    extern crate env_logger;
    extern crate log;

    // pub fn execute(vm_stack_id: u8,vm:&mut Vm) {
    //     //let mut vm = GLOBAL_VM.lock().unwrap();
    //     // 使用指针获取多个可变引用
    //     let vm_stack = vm.vm_stack.get_mut(&vm_stack_id).unwrap() as *mut _;
    //     let heap = &mut vm.heap as *mut _;
    //     let metaspace = &mut vm.metaspace as *mut _;
    //     //drop(vm);
    //     // 转换为可变引用（需要 unsafe）
    //     unsafe {
    //         let vm_stack = &mut *vm_stack;
    //         let heap = &mut *heap;
    //         let metaspace = &mut *metaspace;
    //         do_opcode(vm_stack, heap, metaspace);
    //     }
    // }

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
        //let mut map = HashMap::new();
        while !vm_stack.is_empty() {
            //let start = Instant::now();

            //let code = vm_stack.last().unwrap().code[vm_stack.last().unwrap().pc];
            let frame_index = vm_stack.len() - 1;
            let code = vm_stack[frame_index].code[vm_stack[frame_index].pc];
            let frame: &mut StackFrame = &mut vm_stack[frame_index];

            //println!("{:x}--{}--{:?}--{:?}--{:?}--opstack:{:?}--local:{:?}",code,frame.pc,frame.class_name,frame.method_name,frame.descriptor,frame.op_stack,frame.local);
            println!(
                "{:x}--{}--{:?}--{:?}--{:?}",
                code, frame.pc, frame.class_name, frame.method_name, frame.descriptor
            );
            // if code == 0xbb || code == 0xbc || code == 0xbd || code == 0xc5 {
            //     full_gc();
            // }
            let result = match code {
                0x00 => nop(&mut vm_stack[frame_index]),
                0x01 => aconst_null(&mut vm_stack[frame_index]),
                0x02 => iconst_m1(&mut vm_stack[frame_index]),
                0x03 => iconst_0(&mut vm_stack[frame_index]),
                0x04 => iconst_1(&mut vm_stack[frame_index]),
                0x05 => iconst_2(&mut vm_stack[frame_index]),
                0x06 => iconst_3(&mut vm_stack[frame_index]),
                0x07 => iconst_4(&mut vm_stack[frame_index]),
                0x08 => iconst_5(&mut vm_stack[frame_index]),
                0x09 => lconst_0(&mut vm_stack[frame_index]),
                0x0a => lconst_1(&mut vm_stack[frame_index]),
                0x0b => fconst_0(&mut vm_stack[frame_index]),
                0x0c => fconst_1(&mut vm_stack[frame_index]),
                0x0d => fconst_2(&mut vm_stack[frame_index]),
                0x0e => dconst_0(&mut vm_stack[frame_index]),
                0x0f => dconst_1(&mut vm_stack[frame_index]),
                0x10 => bipush(&mut vm_stack[frame_index]),
                0x11 => sipush(&mut vm_stack[frame_index]),
                0x12 => ldc(vm_stack, heap, metaspace),
                0x14 => ldc2_w(vm_stack, heap, metaspace),
                0x15 => iload(&mut vm_stack[frame_index]),
                0x16 => lload(&mut vm_stack[frame_index]),
                0x17 => fload(&mut vm_stack[frame_index]),
                0x18 => dload(&mut vm_stack[frame_index]),
                0x19 => aload(&mut vm_stack[frame_index]),
                0x1a => iload_0(&mut vm_stack[frame_index]),
                0x1b => iload_1(&mut vm_stack[frame_index]),
                0x1c => iload_2(&mut vm_stack[frame_index]),
                0x1d => iload_3(&mut vm_stack[frame_index]),
                0x1e => lload_0(&mut vm_stack[frame_index]),
                0x1f => lload_1(&mut vm_stack[frame_index]),
                0x20 => lload_2(&mut vm_stack[frame_index]),
                0x21 => lload_3(&mut vm_stack[frame_index]),
                0x22 => fload_0(&mut vm_stack[frame_index]),
                0x23 => fload_1(&mut vm_stack[frame_index]),
                0x24 => fload_2(&mut vm_stack[frame_index]),
                0x25 => fload_3(&mut vm_stack[frame_index]),
                0x26 => dload_0(&mut vm_stack[frame_index]),
                0x27 => dload_1(&mut vm_stack[frame_index]),
                0x28 => dload_2(&mut vm_stack[frame_index]),
                0x29 => dload_3(&mut vm_stack[frame_index]),
                0x2a => aload_0(&mut vm_stack[frame_index]),
                0x2b => aload_1(&mut vm_stack[frame_index]),
                0x2c => aload_2(&mut vm_stack[frame_index]),
                0x2d => aload_3(&mut vm_stack[frame_index]),
                0x2e => iaload(vm_stack, heap, metaspace),
                0x2f => laload(vm_stack, heap, metaspace),
                0x30 => faload(vm_stack, heap, metaspace),
                0x31 => daload(vm_stack, heap, metaspace),
                0x32 => aaload(vm_stack, heap, metaspace),
                0x33 => baload(vm_stack, heap, metaspace),
                0x34 => caload(vm_stack, heap, metaspace),
                0x35 => saload(vm_stack, heap, metaspace),
                0x36 => istore(&mut vm_stack[frame_index]),
                0x37 => lstore(&mut vm_stack[frame_index]),
                0x38 => fstore(&mut vm_stack[frame_index]),
                0x39 => dstore(&mut vm_stack[frame_index]),
                0x3a => astore(&mut vm_stack[frame_index]),
                0x3b => istore_0(&mut vm_stack[frame_index]),
                0x3c => istore_1(&mut vm_stack[frame_index]),
                0x3d => istore_2(&mut vm_stack[frame_index]),
                0x3e => istore_3(&mut vm_stack[frame_index]),
                0x3f => lstore_0(&mut vm_stack[frame_index]),
                0x40 => lstore_1(&mut vm_stack[frame_index]),
                0x41 => lstore_2(&mut vm_stack[frame_index]),
                0x42 => lstore_3(&mut vm_stack[frame_index]),
                0x43 => fstore_0(&mut vm_stack[frame_index]),
                0x44 => fstore_1(&mut vm_stack[frame_index]),
                0x45 => fstore_2(&mut vm_stack[frame_index]),
                0x46 => fstore_3(&mut vm_stack[frame_index]),
                0x47 => dstore_0(&mut vm_stack[frame_index]),
                0x48 => dstore_1(&mut vm_stack[frame_index]),
                0x49 => dstore_2(&mut vm_stack[frame_index]),
                0x4a => dstore_3(&mut vm_stack[frame_index]),
                0x4b => astore_0(&mut vm_stack[frame_index]),
                0x4c => astore_1(&mut vm_stack[frame_index]),
                0x4d => astore_2(&mut vm_stack[frame_index]),
                0x4e => astore_3(&mut vm_stack[frame_index]),
                0x4f => iastore(vm_stack, heap, metaspace),
                0x50 => lastore(vm_stack, heap, metaspace),
                0x51 => fastore(vm_stack, heap, metaspace),
                0x52 => dastore(vm_stack, heap, metaspace),
                0x53 => aastore(vm_stack, heap, metaspace),
                0x54 => bastore(vm_stack, heap, metaspace),
                0x55 => castore(vm_stack, heap, metaspace),
                0x56 => sastore(vm_stack, heap, metaspace),
                0x57 => pop(&mut vm_stack[frame_index]),
                0x58 => pop2(&mut vm_stack[frame_index]),
                0x59 => dup(&mut vm_stack[frame_index]),
                0x5a => dup_x1(&mut vm_stack[frame_index]),
                0x5b => dup_x2(&mut vm_stack[frame_index]),
                0x5c => dup2(&mut vm_stack[frame_index]),
                0x5d => dup2_x1(&mut vm_stack[frame_index]),
                0x5e => dup2_x2(&mut vm_stack[frame_index]),
                0x5f => swap(&mut vm_stack[frame_index]),
                0x60 => iadd(&mut vm_stack[frame_index]),
                0x61 => ladd(&mut vm_stack[frame_index]),
                0x62 => fadd(&mut vm_stack[frame_index]),
                0x63 => dadd(&mut vm_stack[frame_index]),
                0x64 => isub(&mut vm_stack[frame_index]),
                0x65 => lsub(&mut vm_stack[frame_index]),
                0x66 => fsub(&mut vm_stack[frame_index]),
                0x67 => dsub(&mut vm_stack[frame_index]),
                0x68 => imul(&mut vm_stack[frame_index]),
                0x69 => lmul(&mut vm_stack[frame_index]),
                0x6a => fmul(&mut vm_stack[frame_index]),
                0x6b => dmul(&mut vm_stack[frame_index]),
                0x6c => idiv(&mut vm_stack[frame_index]),
                0x6d => ldiv(&mut vm_stack[frame_index]),
                0x6e => fdiv(&mut vm_stack[frame_index]),
                0x6f => ddiv(&mut vm_stack[frame_index]),
                0x70 => irem(&mut vm_stack[frame_index]),
                0x71 => lrem(&mut vm_stack[frame_index]),
                0x72 => frem(&mut vm_stack[frame_index]),
                0x73 => drem(&mut vm_stack[frame_index]),
                0x74 => ineg(&mut vm_stack[frame_index]),
                0x75 => lneg(&mut vm_stack[frame_index]),
                0x76 => fneg(&mut vm_stack[frame_index]),
                0x77 => dneg(&mut vm_stack[frame_index]),
                0x78 => ishl(&mut vm_stack[frame_index]),
                0x79 => lshl(&mut vm_stack[frame_index]),
                0x7a => ishr(&mut vm_stack[frame_index]),
                0x7b => lshr(&mut vm_stack[frame_index]),
                0x7c => iushr(&mut vm_stack[frame_index]),
                0x7d => lushr(&mut vm_stack[frame_index]),
                0x7e => iand(&mut vm_stack[frame_index]),
                0x7f => land(&mut vm_stack[frame_index]),
                0x80 => ior(&mut vm_stack[frame_index]),
                0x81 => lor(&mut vm_stack[frame_index]),
                0x82 => ixor(&mut vm_stack[frame_index]),
                0x83 => lxor(&mut vm_stack[frame_index]),
                0x84 => iinc(&mut vm_stack[frame_index]),
                0x85 => i2l(&mut vm_stack[frame_index]),
                0x86 => i2f(&mut vm_stack[frame_index]),
                0x87 => i2d(&mut vm_stack[frame_index]),
                0x88 => l2i(&mut vm_stack[frame_index]),
                0x89 => l2f(&mut vm_stack[frame_index]),
                0x8a => l2d(&mut vm_stack[frame_index]),
                0x8b => f2i(&mut vm_stack[frame_index]),
                0x8c => f2l(&mut vm_stack[frame_index]),
                0x8d => f2d(&mut vm_stack[frame_index]),
                0x8e => d2i(&mut vm_stack[frame_index]),
                0x8f => d2l(&mut vm_stack[frame_index]),
                0x90 => d2f(&mut vm_stack[frame_index]),
                0x91 => i2b(&mut vm_stack[frame_index]),
                0x92 => i2c(&mut vm_stack[frame_index]),
                0x93 => i2s(&mut vm_stack[frame_index]),
                0x94 => lcmp(&mut vm_stack[frame_index]),
                0x95 => fcmpl(&mut vm_stack[frame_index]),
                0x96 => fcmpg(&mut vm_stack[frame_index]),
                0x97 => dcmpl(&mut vm_stack[frame_index]),
                0x98 => dcmpg(&mut vm_stack[frame_index]),
                0x99 => ifeq(&mut vm_stack[frame_index]),
                0x9a => ifne(&mut vm_stack[frame_index]),
                0x9b => iflt(&mut vm_stack[frame_index]),
                0x9c => ifge(&mut vm_stack[frame_index]),
                0x9d => ifgt(&mut vm_stack[frame_index]),
                0x9e => ifle(&mut vm_stack[frame_index]),
                0x9f => if_icmpeq(&mut vm_stack[frame_index]),
                0xa0 => if_icmpne(&mut vm_stack[frame_index]),
                0xa1 => if_icmplt(&mut vm_stack[frame_index]),
                0xa2 => if_icmpge(&mut vm_stack[frame_index]),
                0xa3 => if_icmpgt(&mut vm_stack[frame_index]),
                0xa4 => if_icmple(&mut vm_stack[frame_index]),
                0xa5 => if_acmpeq(&mut vm_stack[frame_index]),
                0xa6 => if_acmpne(&mut vm_stack[frame_index]),
                0xa7 => goto(&mut vm_stack[frame_index]),
                // 0xa8 => jsr(&mut vm_stack[frame_index]),
                // 0xa9 => ret(&mut vm_stack[frame_index]),
                0xaa => tableswitch(&mut vm_stack[frame_index]),
                0xab => lookupswitch(&mut vm_stack[frame_index]),
                0xac => ireturn(vm_stack),
                0xad => lreturn(vm_stack),
                0xae => freturn(vm_stack),
                0xaf => dreturn(vm_stack),
                0xb0 => areturn(vm_stack),
                0xb1 => _return(vm_stack),
                0xb2 => getstatic(vm_stack, heap, metaspace),
                0xb3 => putstatic(vm_stack, heap, metaspace),
                0xb4 => getfield(vm_stack, heap, metaspace),
                0xb5 => putfield(vm_stack, heap, metaspace),
                0xb6 => invokevirtual(vm_stack, heap, metaspace),
                0xb7 => invokespecial(vm_stack, heap, metaspace),
                0xb8 => invokestatic(vm_stack, heap, metaspace),
                0xb9 => invokeinterface(vm_stack, heap, metaspace),
                // 0xba => invokedynamic(&mut vm_stack[frame_index]),
                0xbb => _new(vm_stack, heap, metaspace),
                0xbc => newarray(vm_stack, heap, metaspace),
                0xbd => anewarray(vm_stack, heap, metaspace),
                0xbe => arraylength(vm_stack, heap),
                0xbf => athrow(&mut vm_stack[frame_index]),
                0xc0 => checkcast(&mut vm_stack[frame_index]),
                0xc1 => instanceof(vm_stack, heap, metaspace),
                0xc2 => monitorenter(&mut vm_stack[frame_index]),
                0xc3 => monitorexit(&mut vm_stack[frame_index]),
                // 0xc4 => wide(&mut vm_stack[frame_index]),
                0xc5 => multianewarray(vm_stack, heap, metaspace),
                0xc6 => ifnull(&mut vm_stack[frame_index]),
                0xc7 => ifnonnull(&mut vm_stack[frame_index]),
                // 0xc8 => goto_w(&mut vm_stack[frame_index]),
                // 0xc9 => jsr_w(&mut vm_stack[frame_index]),
                _ => {
                    // 处理未知指令的情况，可以抛出错误或执行默认操作
                    panic!("Unknown instruction code: 0x{:02X}", code);
                }
            };
            if vm_stack.len() > frame_index {
                let _ = handle_result(vm_stack, heap, metaspace, code == 0xbf, result);
            }
        }
    }

    pub fn handle_result(
        vm_stack: &mut Vec<StackFrame>,
        heap: &mut Heap,
        metaspace: &mut Metaspace,
        athrow_flag: bool,
        result: Result<(), Throwable>,
    ) -> Result<(), Throwable> {
        match result {
            Ok(()) => {
                return Ok(());
            }
            Err(error) => {
                while !vm_stack.is_empty() {
                    let mut stack_frame = vm_stack.pop().unwrap();
                    let mut exception_object_id: Option<u32> = None;
                    let mut exception_class_name: Option<String> = None;
                    let mut solved_flag = false;
                    if athrow_flag {
                        let e: Option<&StackFrameValue> = stack_frame.op_stack.last();
                        match e {
                            Some(e) => {
                                match e {
                                    StackFrameValue::Reference(id) => {
                                        let class_id = heap.get_class(*id);
                                        let class = &metaspace.classes[class_id as usize];
                                        exception_object_id = Some(*id);
                                        exception_class_name = Some(class.class_name.clone());
                                    }
                                    StackFrameValue::Null => {
                                        if e == &StackFrameValue::Null {
                                            return Err(Throwable::Exception(crate::common::error::Exception::NullPointerException));
                                        }
                                    }
                                    _ => {
                                        return Err(Throwable::Error(
                                            crate::common::error::JvmError::UnknownError,
                                        ))
                                    }
                                }
                            }
                            None => {
                                return Err(Throwable::Error(
                                    crate::common::error::JvmError::UnknownError,
                                ))
                            }
                        }
                    } else {
                        //let mut exception_class_name :Option<&String> = None;
                        match &error {
                            Throwable::Exception(exception) => match exception {
                                crate::common::error::Exception::NullPointerException => {
                                    exception_class_name = Some("java/lang/NullPointerException".to_string());
                                    
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ArrayIndexOutOfBoundsException => {
                                    exception_class_name = Some("java/lang/ArrayIndexOutOfBoundsException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ClassCastException => {
                                    exception_class_name = Some("java/lang/ClassCastException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ArithmeticException => {
                                    exception_class_name = Some("java/lang/ArithmeticException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::IllegalArgumentException => {
                                    exception_class_name = Some("java/lang/IllegalArgumentException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::IllegalStateException => {
                                    exception_class_name = Some("java/lang/IllegalStateException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::IOException => {
                                    exception_class_name = Some("java/io/IOException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::IndexOutOfBoundsException => {
                                    exception_class_name = Some("java/lang/IndexOutOfBoundsException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::NegativeArraySizeException => {
                                    exception_class_name = Some("java/lang/NegativeArraySizeException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::NumberFormatException => {
                                    exception_class_name = Some("java/lang/NumberFormatException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ConcurrentModificationException => {
                                    exception_class_name = Some("java/util/ConcurrentModificationException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::UnsupportedOperationException => {
                                    exception_class_name = Some("java/lang/UnsupportedOperationException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ClassNotFoundException => {
                                    exception_class_name = Some("java/lang/ClassNotFoundException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::FileNotFoundException => {
                                    exception_class_name = Some("java/io/FileNotFoundException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::InterruptedException => {
                                    exception_class_name = Some("java/lang/InterruptedException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::SecurityException => {
                                    exception_class_name = Some("java/lang/SecurityException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::TimeoutException => {
                                    exception_class_name = Some("java/util/concurrent/TimeoutException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ParseException => {
                                    exception_class_name = Some("java/text/ParseException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::SQLException => {
                                    exception_class_name = Some("java/sql/SQLException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::Exception::ReflectiveOperationException => {
                                    exception_class_name = Some("java/lang/ReflectiveOperationException".to_string());
                                    // 创建异常对象的逻辑...
                                }
                            },
                            Throwable::Error(jvm_error) => match jvm_error {
                                crate::common::error::JvmError::OutOfMemoryError => {
                                    exception_class_name = Some("java/lang/OutOfMemoryError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::StackOverflowError => {
                                    exception_class_name = Some("java/lang/StackOverflowError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::InternalError => {
                                    exception_class_name = Some("java/lang/InternalError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::UnknownError => {
                                    exception_class_name = Some("java/lang/UnknownError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::NoClassDefFoundError => {
                                    exception_class_name = Some("java/lang/NoClassDefFoundError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::ClassFormatError => {
                                    exception_class_name = Some("java/lang/ClassFormatError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::UnsupportedClassVersionError => {
                                    exception_class_name = Some("java/lang/UnsupportedClassVersionError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::NoSuchFieldError => {
                                    exception_class_name = Some("java/lang/NoSuchFieldError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::NoSuchMethodError => {
                                    exception_class_name = Some("java/lang/NoSuchMethodError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::AbstractMethodError => {
                                    exception_class_name = Some("java/lang/AbstractMethodError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::IllegalAccessError => {
                                    exception_class_name = Some("java/lang/IllegalAccessError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::InstantiationError => {
                                    exception_class_name = Some("java/lang/InstantiationError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::IncompatibleClassChangeError => {
                                    exception_class_name = Some("java/lang/IncompatibleClassChangeError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::UnsatisfiedLinkError => {
                                    exception_class_name = Some("java/lang/UnsatisfiedLinkError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::VerifyError => {
                                    exception_class_name = Some("java/lang/VerifyError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::BootstrapMethodError => {
                                    exception_class_name = Some("java/lang/BootstrapMethodError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::ThreadDeath => {
                                    exception_class_name = Some("java/lang/ThreadDeath".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::AssertionError => {
                                    exception_class_name = Some("java/lang/AssertionError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::IOError => {
                                    exception_class_name = Some("java/io/IOError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                                crate::common::error::JvmError::ExceptionInInitializerError => {
                                    exception_class_name = Some("java/lang/ExceptionInInitializerError".to_string());
                                    // 创建异常对象的逻辑...
                                }
                            },
                        }
                    }

                    let exception_class = classloader::class_loader::find_class(
                        &exception_class_name.unwrap().clone(),
                        &mut Vec::new(),
                        heap,
                        metaspace,
                    )?
                    .clone();

                    if exception_object_id.is_none(){
                        exception_object_id = Some( heap.create_object(&exception_class)? as u32)
                    }
                    let (method, constant_pool) = {
                        let (method, class) = metaspace.get_method_from_root(
                            &stack_frame.class_name,
                            &stack_frame.method_name,
                            &stack_frame.descriptor,
                        );
                        (method.cloned(), class.constant_pool.clone())
                    };


                    if let Some(method) = method {
                        for i in 0..method.attributes.len() {
                            let attribute: &AttributeInfo = &method.attributes[i];
                            // 判断是否为Code
                            if let AttributeInfo::Code(code_attr) = attribute {
                                for entry in &code_attr.exception_table {
                                    let start_pc = entry.start_pc;
                                    let end_pc = entry.end_pc;
                                    let handler_pc = entry.handler_pc;
                                    let catch_type = entry.catch_type;
                                    // 如果当前PC在异常处理范围内
                                    if stack_frame.pc >= start_pc as usize
                                        && stack_frame.pc <= end_pc as usize
                                    {
                                        let constant_class = &constant_pool[catch_type as usize];
                                        let class_name = match constant_class {
                                            ConstantPoolInfo::Class(name_index) => {
                                                let class_name_utf8 =
                                                    &constant_pool[*name_index as usize];
                                                match class_name_utf8 {
                                                    ConstantPoolInfo::Utf8(name_string) => {
                                                        name_string
                                                    }
                                                    _ => panic!("error"),
                                                }
                                            }
                                            _ => panic!("error"),
                                        };

                                        //异常被捕获
                                        if metaspace.is_subclass(&exception_class.class_name, class_name) || &exception_class.class_name == class_name {
                                            stack_frame.pc = handler_pc as usize;
                                            stack_frame
                                            .op_stack
                                            .push(StackFrameValue::Reference(exception_object_id.unwrap()));
                                            solved_flag = true;
                                            break;
                                        }
                                        
                                    }
                                }
                            }
                        }
                    }

                    if solved_flag {
                        vm_stack.push(stack_frame);
                    }

                }
            }
        };
        Ok(())
    }
}
