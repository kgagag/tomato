pub mod op_code {
    use crate::stack_frame::StackFrame;
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

    pub fn do_opcode(vm_stack: &mut Vec<StackFrame>) {
        while vm_stack.len() > 0 {
            let mut len = (&vm_stack).len();
            let mut stack_frame = vm_stack.get_mut(len - 1).unwrap();
            while stack_frame.pc < stack_frame.code.len() {
                let code = stack_frame.code[stack_frame.pc];
                error!("{:?}",stack_frame.op_stack);
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
                    0x70 => irem(stack_frame),
                    0x71 => lrem(stack_frame),
                    0x72 => frem(stack_frame),
                    0x73 => drem(stack_frame),
                    0x74 => ineg(stack_frame),
                    0x75 => lneg(stack_frame),
                    0x76 => fneg(stack_frame),
                    0x77 => dneg(stack_frame),
                    0x78 => ishl(stack_frame),
                    0x79 => lshl(stack_frame),
                    0x7a => ishr(stack_frame),
                    0x7b => lshr(stack_frame),
                    0x7c => iushr(stack_frame),
                    0x7d => lushr(stack_frame),
                    0x7e => iand(stack_frame),
                    0x7f => land(stack_frame),
                    0x80 => ior(stack_frame),
                    0x81 => lor(stack_frame),
                    0x82 => ixor(stack_frame),
                    0x83 => lxor(stack_frame),
                    0x84 => iinc(stack_frame),
                    0x85 => i2l(stack_frame),
                    0x86 => i2f(stack_frame),
                    0x87 => i2d(stack_frame),
                    0x88 => l2i(stack_frame),
                    0x89 => l2f(stack_frame),
                    0x8a => l2d(stack_frame),
                    0x8b => f2i(stack_frame),
                    0x8c => f2l(stack_frame),
                    0x8d => f2d(stack_frame),
                    0x8e => d2i(stack_frame),
                    0x8f => d2l(stack_frame),
                    0x90 => d2f(stack_frame),
                    0x91 => i2b(stack_frame),
                    0x92 => i2c(stack_frame),
                    0x93 => i2s(stack_frame),
                    0x94 => lcmp(stack_frame),
                    0x95 => fcmpl(stack_frame),
                    0x96 => fcmpg(stack_frame),
                    0x97 => dcmpl(stack_frame),
                    0x98 => dcmpg(stack_frame),
                    0x99 => ifeq(stack_frame),
                    0x9a => ifne(stack_frame),
                    0x9b => iflt(stack_frame),
                    0x9c => ifge(stack_frame),
                    0x9d => ifgt(stack_frame),
                    0x9e => ifle(stack_frame),
                    0x9f => if_icmpeq(stack_frame),
                    0xa0 => if_icmpne(stack_frame),
                    0xa1 => if_icmplt(stack_frame),
                    0xa2 => if_icmpge(stack_frame),
                    0xa3 => if_icmpgt(stack_frame),
                    0xa4 => if_icmple(stack_frame),
                    0xa5 => if_acmpeq(stack_frame),
                    0xa6 => if_acmpne(stack_frame),
                    0xa7 => goto(stack_frame),
                    // 0xa8 => jsr(stack_frame),
                    // 0xa9 => ret(stack_frame),
                    // 0xaa => tableswitch(stack_frame),
                    // 0xab => lookupswitch(stack_frame),
                    0xac => ireturn(stack_frame),
                    0xad => lreturn(stack_frame),
                    0xae => freturn(stack_frame),
                    0xaf => dreturn(stack_frame),
                    0xb0 => areturn(stack_frame),
                    0xb1 => _return(stack_frame),
                    0xb2 => getstatic(stack_frame),
                    0xb3 => putstatic(stack_frame),
                    0xb4 => getfield(stack_frame),
                    0xb5 => putfield(stack_frame),
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
}
