pub mod op_code {
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use std::sync::MutexGuard;

    use log::info;
    use opcode_array::*;
    use opcode_checkcast::*;
    use opcode_compare::*;
    use opcode_const::*;
    use opcode_convert::*;
    use opcode_dup::*;
    use opcode_exception::*;
    use opcode_field::*;
    use opcode_instanceof::*;
    use opcode_invoke::*;
    use opcode_ldc::*;
    use opcode_load::*;
    use opcode_math::*;
    use opcode_new::*;
    use opcode_nop::*;
    use opcode_goto::*;
    use opcode_pop::*;
    use opcode_push::*;
    use opcode_return::*;
    use opcode_static::*;
    use opcode_swap::*;
    use opcode_store::*;
    use opcode_thread::*;
  

    use opcode_nop::nop;
    use crate::common::stack_frame::StackFrame;
    //use crate::memory::gc::full_gc;
    use crate::interpreter::instructions::*;
    extern crate env_logger;
    extern crate log;
   
    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while !vm_stack.is_empty() && vm_stack.last().unwrap().pc < vm_stack.last().unwrap().code.len() {
            let code = vm_stack.last().unwrap().code[vm_stack.last().unwrap().pc];
            //let frame = vm_stack.last_mut().unwrap();
            info!("{:x}--{}--{:?}--{:?}--{:?}--opstack:{:?}--local:{:?}",code,vm_stack.last().unwrap().pc,vm_stack.last().unwrap().class_name,vm_stack.last().unwrap().method_name,vm_stack.last().unwrap().descriptor,vm_stack.last().unwrap().op_stack,vm_stack.last().unwrap().local);
            if code == 0xbb || code == 0xbc || code == 0xbd || code == 0xc5{
              //  full_gc();
            } 
            //info!("{},{}",vm_stack.last().unwrap().method_name,vm_stack.last().unwrap().class_name);
            match code {
                0x00 => nop(vm_stack),
                0x01 => aconst_null(vm_stack),
                0x02 => iconst_m1(vm_stack),
                0x03 => iconst_0(vm_stack),
                0x04 => iconst_1(vm_stack),
                0x05 => iconst_2(vm_stack),
                0x06 => iconst_3(vm_stack),
                0x07 => iconst_4(vm_stack),
                0x08 => iconst_5(vm_stack),
                0x09 => lconst_0(vm_stack),
                0x0a => lconst_1(vm_stack),
                0x0b => fconst_0(vm_stack),
                0x0c => fconst_1(vm_stack),
                0x0d => fconst_2(vm_stack),
                0x0e => dconst_0(vm_stack),
                0x0f => dconst_1(vm_stack),
                0x10 => bipush(vm_stack),
                0x11 => sipush(vm_stack),
                0x12 => ldc(vm_stack),
                0x13 => ldc_w(vm_stack),
                0x14 => ldc2_w(vm_stack),
                0x15 => iload(vm_stack),
                0x16 => lload(vm_stack),
                0x17 => fload(vm_stack),
                0x18 => dload(vm_stack),
                0x19 => aload(vm_stack),
                0x1a => iload_0(vm_stack),
                0x1b => iload_1(vm_stack),
                0x1c => iload_2(vm_stack),
                0x1d => iload_3(vm_stack),
                0x1e => lload_0(vm_stack),
                0x1f => lload_1(vm_stack),
                0x20 => lload_2(vm_stack),
                0x21 => lload_3(vm_stack),
                0x22 => fload_0(vm_stack),
                0x23 => fload_1(vm_stack),
                0x24 => fload_2(vm_stack),
                0x25 => fload_3(vm_stack),
                0x26 => dload_0(vm_stack),
                0x27 => dload_1(vm_stack),
                0x28 => dload_2(vm_stack),
                0x29 => dload_3(vm_stack),
                0x2a => aload_0(vm_stack),
                0x2b => aload_1(vm_stack),
                0x2c => aload_2(vm_stack),
                0x2d => aload_3(vm_stack),
                0x2e => iaload(vm_stack),
                0x2f => laload(vm_stack),
                0x30 => faload(vm_stack),
                0x31 => daload(vm_stack),
                0x32 => aaload(vm_stack),
                0x33 => baload(vm_stack),
                0x34 => caload(vm_stack),
                0x35 => saload(vm_stack),
                0x36 => istore(vm_stack),
                0x37 => lstore(vm_stack),
                0x38 => fstore(vm_stack),
                0x39 => dstore(vm_stack),
                0x3a => astore(vm_stack),
                0x3b => istore_0(vm_stack),
                0x3c => istore_1(vm_stack),
                0x3d => istore_2(vm_stack),
                0x3e => istore_3(vm_stack),
                0x3f => lstore_0(vm_stack),
                0x40 => lstore_1(vm_stack),
                0x41 => lstore_2(vm_stack),
                0x42 => lstore_3(vm_stack),
                0x43 => fstore_0(vm_stack),
                0x44 => fstore_1(vm_stack),
                0x45 => fstore_2(vm_stack),
                0x46 => fstore_3(vm_stack),
                0x47 => dstore_0(vm_stack),
                0x48 => dstore_1(vm_stack),
                0x49 => dstore_2(vm_stack),
                0x4a => dstore_3(vm_stack),
                0x4b => astore_0(vm_stack),
                0x4c => astore_1(vm_stack),
                0x4d => astore_2(vm_stack),
                0x4e => astore_3(vm_stack),
                0x4f => iastore(vm_stack),
                0x50 => lastore(vm_stack),
                0x51 => fastore(vm_stack),
                0x52 => dastore(vm_stack),
                0x53 => aastore(vm_stack),
                0x54 => bastore(vm_stack),
                0x55 => castore(vm_stack),
                0x56 => sastore(vm_stack),
                0x57 => pop(vm_stack),
                0x58 => pop2(vm_stack),
                0x59 => dup(vm_stack),
                0x5a => dup_x1(vm_stack),
                0x5b => dup_x2(vm_stack),
                0x5c => dup2(vm_stack),
                0x5d => dup2_x1(vm_stack),
                0x5e => dup2_x2(vm_stack),
                0x5f => swap(vm_stack),
                0x60 => iadd(vm_stack),
                0x61 => ladd(vm_stack),
                0x62 => fadd(vm_stack),
                0x63 => dadd(vm_stack),
                0x64 => isub(vm_stack),
                0x65 => lsub(vm_stack),
                0x66 => fsub(vm_stack),
                0x67 => dsub(vm_stack),
                0x68 => imul(vm_stack),
                0x69 => lmul(vm_stack),
                0x6a => fmul(vm_stack),
                0x6b => dmul(vm_stack),
                0x6c => idiv(vm_stack),
                0x6d => ldiv(vm_stack),
                0x6e => fdiv(vm_stack),
                0x6f => ddiv(vm_stack),
                0x70 => irem(vm_stack),
                0x71 => lrem(vm_stack),
                0x72 => frem(vm_stack),
                0x73 => drem(vm_stack),
                0x74 => ineg(vm_stack),
                0x75 => lneg(vm_stack),
                0x76 => fneg(vm_stack),
                0x77 => dneg(vm_stack),
                0x78 => ishl(vm_stack),
                0x79 => lshl(vm_stack),
                0x7a => ishr(vm_stack),
                0x7b => lshr(vm_stack),
                0x7c => iushr(vm_stack),
                0x7d => lushr(vm_stack),
                0x7e => iand(vm_stack),
                0x7f => land(vm_stack),
                0x80 => ior(vm_stack),
                0x81 => lor(vm_stack),
                0x82 => ixor(vm_stack),
                0x83 => lxor(vm_stack),
                0x84 => iinc(vm_stack),
                0x85 => i2l(vm_stack),
                0x86 => i2f(vm_stack),
                0x87 => i2d(vm_stack),
                0x88 => l2i(vm_stack),
                0x89 => l2f(vm_stack),
                0x8a => l2d(vm_stack),
                0x8b => f2i(vm_stack),
                0x8c => f2l(vm_stack),
                0x8d => f2d(vm_stack),
                0x8e => d2i(vm_stack),
                0x8f => d2l(vm_stack),
                0x90 => d2f(vm_stack),
                0x91 => i2b(vm_stack),
                0x92 => i2c(vm_stack),
                0x93 => i2s(vm_stack),
                0x94 => lcmp(vm_stack),
                0x95 => fcmpl(vm_stack),
                0x96 => fcmpg(vm_stack),
                0x97 => dcmpl(vm_stack),
                0x98 => dcmpg(vm_stack),
                0x99 => ifeq(vm_stack),
                0x9a => ifne(vm_stack),
                0x9b => iflt(vm_stack),
                0x9c => ifge(vm_stack),
                0x9d => ifgt(vm_stack),
                0x9e => ifle(vm_stack),
                0x9f => if_icmpeq(vm_stack),
                0xa0 => if_icmpne(vm_stack),
                0xa1 => if_icmplt(vm_stack),
                0xa2 => if_icmpge(vm_stack),
                0xa3 => if_icmpgt(vm_stack),
                0xa4 => if_icmple(vm_stack),
                0xa5 => if_acmpeq(vm_stack),
                0xa6 => if_acmpne(vm_stack),
                0xa7 => goto(vm_stack),
                // 0xa8 => jsr(vm_stack),
                // 0xa9 => ret(vm_stack),
                0xaa => tableswitch(vm_stack),
                0xab => lookupswitch(vm_stack),
                0xac => ireturn(vm_stack),
                0xad => lreturn(vm_stack),
                0xae => freturn(vm_stack),
                0xaf => dreturn(vm_stack),
                0xb0 => areturn(vm_stack),
                0xb1 => _return(vm_stack),
                0xb2 => getstatic(vm_stack),
                0xb3 => putstatic(vm_stack),
                0xb4 => getfield(vm_stack),
                0xb5 => putfield(vm_stack),
                0xb6 => invokevirtual(vm_stack),
                0xb7 => invokespecial(vm_stack),
                0xb8 => invokestatic(vm_stack),
                0xb9 => invokeinterface(vm_stack),
                // 0xba => invokedynamic(vm_stack),
                0xbb => _new(vm_stack),
                0xbc => newarray(vm_stack),
                0xbd => anewarray(vm_stack),
                0xbe => arraylength(vm_stack),
                0xbf => athrow(vm_stack),
                0xc0 => checkcast(vm_stack),
                0xc1 => instanceof(vm_stack),
                0xc2 => monitorenter(vm_stack),
                0xc3 => monitorexit(vm_stack),
                // 0xc4 => wide(vm_stack),
                 0xc5 => multianewarray(vm_stack),
                 0xc6 => ifnull(vm_stack),
                 0xc7 => ifnonnull(vm_stack),
                // 0xc8 => goto_w(vm_stack),
                // 0xc9 => jsr_w(vm_stack),
                _ => {
                    // 处理未知指令的情况，可以抛出错误或执行默认操作
                    panic!("Unknown instruction code: 0x{:02X}", code);
                }
            }
            // if(!vm_stack.is_empty()){
            //     frame = frame
            // }else {
            //     break;
            // }
        }
    }
}
