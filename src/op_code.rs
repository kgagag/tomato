pub mod op_code {
    use crate::stack_frame::*;
    extern crate env_logger;
    extern crate log;
    use crate::opcode_array::*;
    use crate::opcode_compare::*;
    use crate::opcode_const::*;
    use crate::opcode_convert::*;
    use crate::opcode_dup::*;
    use crate::opcode_field::*;
    use crate::opcode_goto::*;
    use crate::opcode_invoke::*;
    use crate::opcode_ldc::*;
    use crate::opcode_load::*;
    use crate::opcode_math::*;
    use crate::opcode_new::*;
    use crate::opcode_nop::*;
    use crate::opcode_pop::*;
    use crate::opcode_push::*;
    use crate::opcode_return::*;
    use crate::opcode_static::*;
    use crate::opcode_store::*;
    use crate::opcode_swap::*;
    use log::{error, info, warn};
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use crate::runtime_data_area::VM_STACKS;

    pub fn execute(){
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = VM_STACKS.lock().unwrap();
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            drop(data);
            for (_vm_stack_id, stack_frames) in map {
                //这里可以启动一个线程
                for _i in 0 .. stack_frames.len() as usize{
                    do_opcode(stack_frames);
                }
            }
            
        }
    }

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while vm_stack.len() > 0 && vm_stack.last().unwrap().pc < vm_stack.last().unwrap().code.len() {
            let code = vm_stack.last().unwrap().code[vm_stack.last().unwrap().pc];
            //info!("{:#x}",code);
            match code {
                0x00 => nop(vm_stack.last_mut().unwrap()),
                0x01 => aconst_null(vm_stack.last_mut().unwrap()),
                0x02 => iconst_m1(vm_stack.last_mut().unwrap()),
                0x03 => iconst_0(vm_stack.last_mut().unwrap()),
                0x04 => iconst_1(vm_stack.last_mut().unwrap()),
                0x05 => iconst_2(vm_stack.last_mut().unwrap()),
                0x06 => iconst_3(vm_stack.last_mut().unwrap()),
                0x07 => iconst_4(vm_stack.last_mut().unwrap()),
                0x08 => iconst_5(vm_stack.last_mut().unwrap()),
                0x09 => lconst_0(vm_stack.last_mut().unwrap()),
                0x0a => lconst_1(vm_stack.last_mut().unwrap()),
                0x0b => fconst_0(vm_stack.last_mut().unwrap()),
                0x0c => fconst_1(vm_stack.last_mut().unwrap()),
                0x0d => fconst_2(vm_stack.last_mut().unwrap()),
                0x0e => dconst_0(vm_stack.last_mut().unwrap()),
                0x0f => dconst_1(vm_stack.last_mut().unwrap()),
                0x10 => bipush(vm_stack.last_mut().unwrap()),
                0x11 => sipush(vm_stack.last_mut().unwrap()),
                0x12 => ldc(vm_stack.last_mut().unwrap()),
                0x13 => ldc_w(vm_stack.last_mut().unwrap()),
                0x14 => ldc2_w(vm_stack.last_mut().unwrap()),
                0x15 => iload(vm_stack.last_mut().unwrap()),
                0x16 => lload(vm_stack.last_mut().unwrap()),
                0x17 => fload(vm_stack.last_mut().unwrap()),
                0x18 => dload(vm_stack.last_mut().unwrap()),
                0x19 => aload(vm_stack.last_mut().unwrap()),
                0x1a => iload_0(vm_stack.last_mut().unwrap()),
                0x1b => iload_1(vm_stack.last_mut().unwrap()),
                0x1c => iload_2(vm_stack.last_mut().unwrap()),
                0x1d => iload_3(vm_stack.last_mut().unwrap()),
                0x1e => lload_0(vm_stack.last_mut().unwrap()),
                0x1f => lload_1(vm_stack.last_mut().unwrap()),
                0x20 => lload_2(vm_stack.last_mut().unwrap()),
                0x21 => lload_3(vm_stack.last_mut().unwrap()),
                0x22 => fload_0(vm_stack.last_mut().unwrap()),
                0x23 => fload_1(vm_stack.last_mut().unwrap()),
                0x24 => fload_2(vm_stack.last_mut().unwrap()),
                0x25 => fload_3(vm_stack.last_mut().unwrap()),
                0x26 => dload_0(vm_stack.last_mut().unwrap()),
                0x27 => dload_1(vm_stack.last_mut().unwrap()),
                0x28 => dload_2(vm_stack.last_mut().unwrap()),
                0x29 => dload_3(vm_stack.last_mut().unwrap()),
                0x2a => aload_0(vm_stack.last_mut().unwrap()),
                0x2b => aload_1(vm_stack.last_mut().unwrap()),
                0x2c => aload_2(vm_stack.last_mut().unwrap()),
                0x2d => aload_3(vm_stack.last_mut().unwrap()),
                0x2e => iaload(vm_stack.last_mut().unwrap()),
                0x2f => laload(vm_stack.last_mut().unwrap()),
                0x30 => faload(vm_stack.last_mut().unwrap()),
                0x31 => daload(vm_stack.last_mut().unwrap()),
                0x32 => aaload(vm_stack.last_mut().unwrap()),
                0x33 => baload(vm_stack.last_mut().unwrap()),
                0x34 => caload(vm_stack.last_mut().unwrap()),
                0x35 => saload(vm_stack.last_mut().unwrap()),
                0x36 => istore(vm_stack.last_mut().unwrap()),
                0x37 => lstore(vm_stack.last_mut().unwrap()),
                0x38 => fstore(vm_stack.last_mut().unwrap()),
                0x39 => dstore(vm_stack.last_mut().unwrap()),
                0x3a => astore(vm_stack.last_mut().unwrap()),
                0x3b => istore_0(vm_stack.last_mut().unwrap()),
                0x3c => istore_1(vm_stack.last_mut().unwrap()),
                0x3d => istore_2(vm_stack.last_mut().unwrap()),
                0x3e => istore_3(vm_stack.last_mut().unwrap()),
                0x3f => lstore_0(vm_stack.last_mut().unwrap()),
                0x40 => lstore_1(vm_stack.last_mut().unwrap()),
                0x41 => lstore_2(vm_stack.last_mut().unwrap()),
                0x42 => lstore_3(vm_stack.last_mut().unwrap()),
                0x43 => fstore_0(vm_stack.last_mut().unwrap()),
                0x44 => fstore_1(vm_stack.last_mut().unwrap()),
                0x45 => fstore_2(vm_stack.last_mut().unwrap()),
                0x46 => fstore_3(vm_stack.last_mut().unwrap()),
                0x47 => dstore_0(vm_stack.last_mut().unwrap()),
                0x48 => dstore_1(vm_stack.last_mut().unwrap()),
                0x49 => dstore_2(vm_stack.last_mut().unwrap()),
                0x4a => dstore_3(vm_stack.last_mut().unwrap()),
                0x4b => astore_0(vm_stack.last_mut().unwrap()),
                0x4c => astore_1(vm_stack.last_mut().unwrap()),
                0x4d => astore_2(vm_stack.last_mut().unwrap()),
                0x4e => astore_3(vm_stack.last_mut().unwrap()),
                0x4f => iastore(vm_stack.last_mut().unwrap()),
                0x50 => lastore(vm_stack.last_mut().unwrap()),
                0x51 => fastore(vm_stack.last_mut().unwrap()),
                0x52 => dastore(vm_stack.last_mut().unwrap()),
                0x53 => aastore(vm_stack.last_mut().unwrap()),
                0x54 => bastore(vm_stack.last_mut().unwrap()),
                0x55 => castore(vm_stack.last_mut().unwrap()),
                0x56 => sastore(vm_stack.last_mut().unwrap()),
                0x57 => pop(vm_stack.last_mut().unwrap()),
                0x58 => pop2(vm_stack.last_mut().unwrap()),
                0x59 => dup(vm_stack.last_mut().unwrap()),
                0x5a => dup_x1(vm_stack.last_mut().unwrap()),
                0x5b => dup_x2(vm_stack.last_mut().unwrap()),
                0x5c => dup2(vm_stack.last_mut().unwrap()),
                0x5d => dup2_x1(vm_stack.last_mut().unwrap()),
                0x5e => dup2_x2(vm_stack.last_mut().unwrap()),
                0x5f => swap(vm_stack.last_mut().unwrap()),
                0x60 => iadd(vm_stack.last_mut().unwrap()),
                0x61 => ladd(vm_stack.last_mut().unwrap()),
                0x62 => fadd(vm_stack.last_mut().unwrap()),
                0x63 => dadd(vm_stack.last_mut().unwrap()),
                0x64 => isub(vm_stack.last_mut().unwrap()),
                0x65 => lsub(vm_stack.last_mut().unwrap()),
                0x66 => fsub(vm_stack.last_mut().unwrap()),
                0x67 => dsub(vm_stack.last_mut().unwrap()),
                0x68 => imul(vm_stack.last_mut().unwrap()),
                0x69 => lmul(vm_stack.last_mut().unwrap()),
                0x6a => fmul(vm_stack.last_mut().unwrap()),
                0x6b => dmul(vm_stack.last_mut().unwrap()),
                0x6c => idiv(vm_stack.last_mut().unwrap()),
                0x6d => ldiv(vm_stack.last_mut().unwrap()),
                0x6e => fdiv(vm_stack.last_mut().unwrap()),
                0x6f => ddiv(vm_stack.last_mut().unwrap()),
                0x70 => irem(vm_stack.last_mut().unwrap()),
                0x71 => lrem(vm_stack.last_mut().unwrap()),
                0x72 => frem(vm_stack.last_mut().unwrap()),
                0x73 => drem(vm_stack.last_mut().unwrap()),
                0x74 => ineg(vm_stack.last_mut().unwrap()),
                0x75 => lneg(vm_stack.last_mut().unwrap()),
                0x76 => fneg(vm_stack.last_mut().unwrap()),
                0x77 => dneg(vm_stack.last_mut().unwrap()),
                0x78 => ishl(vm_stack.last_mut().unwrap()),
                0x79 => lshl(vm_stack.last_mut().unwrap()),
                0x7a => ishr(vm_stack.last_mut().unwrap()),
                0x7b => lshr(vm_stack.last_mut().unwrap()),
                0x7c => iushr(vm_stack.last_mut().unwrap()),
                0x7d => lushr(vm_stack.last_mut().unwrap()),
                0x7e => iand(vm_stack.last_mut().unwrap()),
                0x7f => land(vm_stack.last_mut().unwrap()),
                0x80 => ior(vm_stack.last_mut().unwrap()),
                0x81 => lor(vm_stack.last_mut().unwrap()),
                0x82 => ixor(vm_stack.last_mut().unwrap()),
                0x83 => lxor(vm_stack.last_mut().unwrap()),
                0x84 => iinc(vm_stack.last_mut().unwrap()),
                0x85 => i2l(vm_stack.last_mut().unwrap()),
                0x86 => i2f(vm_stack.last_mut().unwrap()),
                0x87 => i2d(vm_stack.last_mut().unwrap()),
                0x88 => l2i(vm_stack.last_mut().unwrap()),
                0x89 => l2f(vm_stack.last_mut().unwrap()),
                0x8a => l2d(vm_stack.last_mut().unwrap()),
                0x8b => f2i(vm_stack.last_mut().unwrap()),
                0x8c => f2l(vm_stack.last_mut().unwrap()),
                0x8d => f2d(vm_stack.last_mut().unwrap()),
                0x8e => d2i(vm_stack.last_mut().unwrap()),
                0x8f => d2l(vm_stack.last_mut().unwrap()),
                0x90 => d2f(vm_stack.last_mut().unwrap()),
                0x91 => i2b(vm_stack.last_mut().unwrap()),
                0x92 => i2c(vm_stack.last_mut().unwrap()),
                0x93 => i2s(vm_stack.last_mut().unwrap()),
                0x94 => lcmp(vm_stack.last_mut().unwrap()),
                0x95 => fcmpl(vm_stack.last_mut().unwrap()),
                0x96 => fcmpg(vm_stack.last_mut().unwrap()),
                0x97 => dcmpl(vm_stack.last_mut().unwrap()),
                0x98 => dcmpg(vm_stack.last_mut().unwrap()),
                0x99 => ifeq(vm_stack.last_mut().unwrap()),
                0x9a => ifne(vm_stack.last_mut().unwrap()),
                0x9b => iflt(vm_stack.last_mut().unwrap()),
                0x9c => ifge(vm_stack.last_mut().unwrap()),
                0x9d => ifgt(vm_stack.last_mut().unwrap()),
                0x9e => ifle(vm_stack.last_mut().unwrap()),
                0x9f => if_icmpeq(vm_stack.last_mut().unwrap()),
                0xa0 => if_icmpne(vm_stack.last_mut().unwrap()),
                0xa1 => if_icmplt(vm_stack.last_mut().unwrap()),
                0xa2 => if_icmpge(vm_stack.last_mut().unwrap()),
                0xa3 => if_icmpgt(vm_stack.last_mut().unwrap()),
                0xa4 => if_icmple(vm_stack.last_mut().unwrap()),
                0xa5 => if_acmpeq(vm_stack.last_mut().unwrap()),
                0xa6 => if_acmpne(vm_stack.last_mut().unwrap()),
                0xa7 => goto(vm_stack.last_mut().unwrap()),
                // 0xa8 => jsr(vm_stack.last_mut().unwrap()),
                // 0xa9 => ret(vm_stack.last_mut().unwrap()),
                // 0xaa => tableswitch(vm_stack.last_mut().unwrap()),
                // 0xab => lookupswitch(vm_stack.last_mut().unwrap()),
                0xac => ireturn(vm_stack.last_mut().unwrap()),
                0xad => lreturn(vm_stack.last_mut().unwrap()),
                0xae => freturn(vm_stack.last_mut().unwrap()),
                0xaf => dreturn(vm_stack.last_mut().unwrap()),
                0xb0 => areturn(vm_stack.last_mut().unwrap()),
                0xb1 => _return(vm_stack.last_mut().unwrap()),
                0xb2 => getstatic(vm_stack.last_mut().unwrap()),
                0xb3 => putstatic(vm_stack.last_mut().unwrap()),
                0xb4 => getfield(vm_stack.last_mut().unwrap()),
                0xb5 => putfield(vm_stack.last_mut().unwrap()),
                0xb6 => invokevirtual(vm_stack.last_mut().unwrap()),
                0xb7 => invokespecial(vm_stack.last_mut().unwrap()),
                0xb8 => invokestatic(vm_stack.last_mut().unwrap()),
                // 0xb9 => invokeinterface(vm_stack.last_mut().unwrap()),
                // 0xba => invokedynamic(vm_stack.last_mut().unwrap()),
                0xbb => _new(vm_stack.last_mut().unwrap()),
                0xbc => newarray(vm_stack.last_mut().unwrap()),
                // 0xbd => anewarray(vm_stack.last_mut().unwrap()),
                // 0xbe => arraylength(vm_stack.last_mut().unwrap()),
                // 0xbf => athrow(vm_stack.last_mut().unwrap()),
                // 0xc0 => checkcast(vm_stack.last_mut().unwrap()),
                // 0xc1 => instanceof(vm_stack.last_mut().unwrap()),
                // 0xc2 => monitorenter(vm_stack.last_mut().unwrap()),
                // 0xc3 => monitorexit(vm_stack.last_mut().unwrap()),
                // 0xc4 => wide(vm_stack.last_mut().unwrap()),
                // 0xc5 => multianewarray(vm_stack.last_mut().unwrap()),
                // 0xc6 => ifnull(vm_stack.last_mut().unwrap()),
                // 0xc7 => ifnonnull(vm_stack.last_mut().unwrap()),
                // 0xc8 => goto_w(vm_stack.last_mut().unwrap()),
                // 0xc9 => jsr_w(vm_stack.last_mut().unwrap()),
                _ => {
                    // 处理未知指令的情况，可以抛出错误或执行默认操作
                    panic!("Unknown instruction code: 0x{:02X}", code);
                }
            }
            // if(!vm_stack.is_empty()){
            //     vm_stack.last_mut().unwrap() = vm_stack.last_mut().unwrap()
            // }else {
            //     break;
            // }
        }
    }
}
