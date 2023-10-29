pub mod stack_frame{
use std::mem;
use std::cell::UnsafeCell;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::u8c::u8c::u8s_to_u16;
use crate::u8c::u8c::u8s_to_u32;
use crate::u8c::u8c::u8s_to_u64;
use crate::class_loader::class_loader::load_class;
use crate::class::class::Class;
use crate::class::class::MethodInfo;
use crate::class::class::MethodParameter;
use crate::class::class::CodeAttribute;
use crate::value::value::StackFrameValue;
/**
 * 栈桢
 */
#[derive(Debug, Clone)]
pub struct StackFrame {
    // //指令
    // pub code: Vec<u8>,
    //程序计数器
    pub pc: usize,
    // //局部变量表
    pub local: Vec<StackFrameValue>,
    // //操作数栈
    pub op_stack: Vec<StackFrameValue>,
    // //类
    pub class: usize,
   
    pub max_stack: u16,

    pub max_locals: u16,

    pub code: Vec<u8>,

    //所属虚拟机栈id
    pub vm_stack_id:u32
}

impl StackFrame {
   
   pub fn new(class: usize, max_stack: u16, max_locals: u16, code: Vec<u8>) -> StackFrame {
       let mut stake_frame =  StackFrame {
            pc: 0,
            class,
            local: Vec::new(),
            op_stack: Vec::new(),
            max_stack,
            max_locals,
            code,
            vm_stack_id:0
        };
        for i in 0 .. stake_frame.max_locals as usize{
            stake_frame.local.push(StackFrameValue::Byte(0));
        }
        return stake_frame;
    }   
}

}