pub mod op_code {
    use log::info;

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
    
    use std::cell::UnsafeCell;
    use std::collections::HashMap;
    use crate::runtime_data_area::VM_STACKS;

    pub fn execute(vm_stack_id : u32){
        let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = VM_STACKS.lock().unwrap();
        unsafe {
            // 从 UnsafeCell 中获取 HashMap 的可变引用
            let map = &mut *data.get();
            drop(data);
            let stack_frames_op = map.get_mut(&vm_stack_id);
            if !stack_frames_op.is_none() {
                let stack_frames = stack_frames_op.unwrap();
                if stack_frames.len() > 0 {
                    do_opcode( stack_frames);
                }
            }
        }
    }

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while !vm_stack.is_empty() && vm_stack.last().unwrap().pc < vm_stack.last().unwrap().code.len() {
            let code = vm_stack.last().unwrap().code[vm_stack.last().unwrap().pc];
            let frame = vm_stack.last_mut().unwrap();
            //info!("{:x}",code);
            match code {
                0x00 => nop(frame),
                0x01 => aconst_null(frame),
                0x02 => iconst_m1(frame),
                0x03 => iconst_0(frame),
                0x04 => iconst_1(frame),
                0x05 => iconst_2(frame),
                0x06 => iconst_3(frame),
                0x07 => iconst_4(frame),
                0x08 => iconst_5(frame),
                0x09 => lconst_0(frame),
                0x0a => lconst_1(frame),
                0x0b => fconst_0(frame),
                0x0c => fconst_1(frame),
                0x0d => fconst_2(frame),
                0x0e => dconst_0(frame),
                0x0f => dconst_1(frame),
                0x10 => bipush(frame),
                0x11 => sipush(frame),
                0x12 => ldc(frame),
                0x13 => ldc_w(frame),
                0x14 => ldc2_w(frame),
                0x15 => iload(frame),
                0x16 => lload(frame),
                0x17 => fload(frame),
                0x18 => dload(frame),
                0x19 => aload(frame),
                0x1a => iload_0(frame),
                0x1b => iload_1(frame),
                0x1c => iload_2(frame),
                0x1d => iload_3(frame),
                0x1e => lload_0(frame),
                0x1f => lload_1(frame),
                0x20 => lload_2(frame),
                0x21 => lload_3(frame),
                0x22 => fload_0(frame),
                0x23 => fload_1(frame),
                0x24 => fload_2(frame),
                0x25 => fload_3(frame),
                0x26 => dload_0(frame),
                0x27 => dload_1(frame),
                0x28 => dload_2(frame),
                0x29 => dload_3(frame),
                0x2a => aload_0(frame),
                0x2b => aload_1(frame),
                0x2c => aload_2(frame),
                0x2d => aload_3(frame),
                0x2e => iaload(frame),
                0x2f => laload(frame),
                0x30 => faload(frame),
                0x31 => daload(frame),
                0x32 => aaload(frame),
                0x33 => baload(frame),
                0x34 => caload(frame),
                0x35 => saload(frame),
                0x36 => istore(frame),
                0x37 => lstore(frame),
                0x38 => fstore(frame),
                0x39 => dstore(frame),
                0x3a => astore(frame),
                0x3b => istore_0(frame),
                0x3c => istore_1(frame),
                0x3d => istore_2(frame),
                0x3e => istore_3(frame),
                0x3f => lstore_0(frame),
                0x40 => lstore_1(frame),
                0x41 => lstore_2(frame),
                0x42 => lstore_3(frame),
                0x43 => fstore_0(frame),
                0x44 => fstore_1(frame),
                0x45 => fstore_2(frame),
                0x46 => fstore_3(frame),
                0x47 => dstore_0(frame),
                0x48 => dstore_1(frame),
                0x49 => dstore_2(frame),
                0x4a => dstore_3(frame),
                0x4b => astore_0(frame),
                0x4c => astore_1(frame),
                0x4d => astore_2(frame),
                0x4e => astore_3(frame),
                0x4f => iastore(frame),
                0x50 => lastore(frame),
                0x51 => fastore(frame),
                0x52 => dastore(frame),
                0x53 => aastore(frame),
                0x54 => bastore(frame),
                0x55 => castore(frame),
                0x56 => sastore(frame),
                0x57 => pop(frame),
                0x58 => pop2(frame),
                0x59 => dup(frame),
                0x5a => dup_x1(frame),
                0x5b => dup_x2(frame),
                0x5c => dup2(frame),
                0x5d => dup2_x1(frame),
                0x5e => dup2_x2(frame),
                0x5f => swap(frame),
                0x60 => iadd(frame),
                0x61 => ladd(frame),
                0x62 => fadd(frame),
                0x63 => dadd(frame),
                0x64 => isub(frame),
                0x65 => lsub(frame),
                0x66 => fsub(frame),
                0x67 => dsub(frame),
                0x68 => imul(frame),
                0x69 => lmul(frame),
                0x6a => fmul(frame),
                0x6b => dmul(frame),
                0x6c => idiv(frame),
                0x6d => ldiv(frame),
                0x6e => fdiv(frame),
                0x6f => ddiv(frame),
                0x70 => irem(frame),
                0x71 => lrem(frame),
                0x72 => frem(frame),
                0x73 => drem(frame),
                0x74 => ineg(frame),
                0x75 => lneg(frame),
                0x76 => fneg(frame),
                0x77 => dneg(frame),
                0x78 => ishl(frame),
                0x79 => lshl(frame),
                0x7a => ishr(frame),
                0x7b => lshr(frame),
                0x7c => iushr(frame),
                0x7d => lushr(frame),
                0x7e => iand(frame),
                0x7f => land(frame),
                0x80 => ior(frame),
                0x81 => lor(frame),
                0x82 => ixor(frame),
                0x83 => lxor(frame),
                0x84 => iinc(frame),
                0x85 => i2l(frame),
                0x86 => i2f(frame),
                0x87 => i2d(frame),
                0x88 => l2i(frame),
                0x89 => l2f(frame),
                0x8a => l2d(frame),
                0x8b => f2i(frame),
                0x8c => f2l(frame),
                0x8d => f2d(frame),
                0x8e => d2i(frame),
                0x8f => d2l(frame),
                0x90 => d2f(frame),
                0x91 => i2b(frame),
                0x92 => i2c(frame),
                0x93 => i2s(frame),
                0x94 => lcmp(frame),
                0x95 => fcmpl(frame),
                0x96 => fcmpg(frame),
                0x97 => dcmpl(frame),
                0x98 => dcmpg(frame),
                0x99 => ifeq(frame),
                0x9a => ifne(frame),
                0x9b => iflt(frame),
                0x9c => ifge(frame),
                0x9d => ifgt(frame),
                0x9e => ifle(frame),
                0x9f => if_icmpeq(frame),
                0xa0 => if_icmpne(frame),
                0xa1 => if_icmplt(frame),
                0xa2 => if_icmpge(frame),
                0xa3 => if_icmpgt(frame),
                0xa4 => if_icmple(frame),
                0xa5 => if_acmpeq(frame),
                0xa6 => if_acmpne(frame),
                0xa7 => goto(frame),
                // 0xa8 => jsr(frame),
                // 0xa9 => ret(frame),
                // 0xaa => tableswitch(frame),
                // 0xab => lookupswitch(frame),
                0xac => ireturn(frame),
                0xad => lreturn(frame),
                0xae => freturn(frame),
                0xaf => dreturn(frame),
                0xb0 => areturn(frame),
                0xb1 => _return(frame),
                0xb2 => getstatic(frame),
                0xb3 => putstatic(frame),
                0xb4 => getfield(frame),
                0xb5 => putfield(frame),
                0xb6 => invokevirtual(frame),
                0xb7 => invokespecial(frame),
                0xb8 => invokestatic(frame),
                0xb9 => invokeinterface(frame),
                // 0xba => invokedynamic(frame),
                0xbb => _new(frame),
                0xbc => newarray(frame),
                0xbd => anewarray(frame),
                 0xbe => arraylength(frame),
                // 0xbf => athrow(frame),
                // 0xc0 => checkcast(frame),
                // 0xc1 => instanceof(frame),
                // 0xc2 => monitorenter(frame),
                // 0xc3 => monitorexit(frame),
                // 0xc4 => wide(frame),
                 0xc5 => multianewarray(frame),
                // 0xc6 => ifnull(frame),
                 0xc7 => ifnonnull(frame),
                // 0xc8 => goto_w(frame),
                // 0xc9 => jsr_w(frame),
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
